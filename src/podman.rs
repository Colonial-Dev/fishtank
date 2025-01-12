use std::collections::HashMap;
use std::process::Command;

use serde::Deserialize;

use crate::prelude::*;
use crate::CommandExt;

pub type Containers = Vec<Container>;
pub type Images     = Vec<Image>;

pub const ANNOTATIONS: [&str; 12] = [
    "args",
    "cap-add",
    "cap-drop",
    "cpus",
    "memory",
    "ulimit",
    "device",
    "userns",
    "security-opt",
    "mount",
    "restart",
    "secret"
];

pub const CONFIG_FLAGS: [&str; 9] = [
    "entrypoint",
    "env",
    "healthcheck",
    "hostname",
    "port",
    "shell",
    "user",
    "volume",
    "workingdir"
];

#[derive(Debug)]
pub struct Container {
    pub id          : String,
    pub image       : String,
    pub state       : String,
    pub annotations : HashMap<String, String>,
}

impl Container {
    pub fn from_id(id: &str) -> Result<Self> {
        #[derive(Debug, Deserialize)]
        struct State {
            #[serde(rename = "Status")]
            status: String
        }

        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(rename = "Annotations")]
            annotations: HashMap<String, String>
        }
        
        #[derive(Debug, Deserialize)]
        struct Raw {
            #[serde(rename = "Id")]
            id    : String,
            #[serde(rename = "State")]
            state : State,
            #[serde(rename = "ImageName")]
            image : String,
            #[serde(rename = "Config")]
            config: Config
        }

        let raw_json = Command::new("podman")
            .args([
                "inspect",
                "--type",
                "container",
                "--format",
                "json",
                id
            ])
            .output_ok()
            .context("Failed to inspect container JSON")?;

        // Podman is weird and always returns the JSON in an array, even when there can only be one element.
        let mut raw: Vec<Raw> = serde_json::from_str(&raw_json)
                .context("Failed to deserialize container JSON")?;

        let Raw { id, image, state, config } = raw
                .pop()
                .expect("Container JSON should always have at least one element");

        Ok(Self {
            id,
            image,
            state       : state.status,
            annotations : config.annotations
        })
    }

    pub fn enumerate() -> Result<Containers> {
        let o = Command::new("podman")
            .args([
                "ps",
                "-a",
                "--format",
                "{{.ID}}"
            ])
            .output_ok()
            .context("Failed to enumerate all container IDs")?;
        
        let mut out = vec![];
                
        for container in o
            .lines()
            .map(Container::from_id) 
        {   
            let container = container?;

            let manager = container
                .annotations
                .get("manager")
                .map(String::as_ref);

            if let Some("box") = manager {
                debug!(
                    "Enumerated container {}",
                    container.id
                );

                out.push(container);
            }
        }
        
        Ok(out)
    }

    pub fn exists(id: &str) -> Result<bool> {
        let output = Command::new("podman")
            .args([
                "container",
                "exists",
                id
            ])
            .output()
            .context("Failed to check if container exists")?;

        Ok(
            output.status.success()
        )
    }

    pub fn started(&self) -> bool {
        self.state == "running"
    }

    pub fn annotation(&self, key: &str) -> Option<&str> {
        self
            .annotations
            .get(key)
            .map(String::as_str)
    }

    pub fn start(&self) -> Result<()> {
        debug!("Starting container {}...", self.id);

        Command::new("podman")
            .arg("start")
            .arg(&self.id)
            .output_ok()
            .context("Failed to start container")?;
        
        Ok(())
    }

    pub fn restart(&self) -> Result<()> {
        debug!("Restarting container {}...", self.id);

        Command::new("podman")
            .args([
                "restart",
                "-t",
                "0"
            ])
            .arg(&self.id)
            .output_ok()
            .context("Failed to restart container")?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        debug!("Stopping container {}...", self.id);

        Command::new("podman")
            .args([
                "stop",
                "-t",
                "0"
            ])
            .arg(&self.id)
            .output_ok()
            .context("Failed to stop container")?;

        Ok(())
    }

    pub fn down(&self) -> Result<()> {
        debug!("Removing container {}...", self.id);
        
        Command::new("podman")
           .args([
                "rm",
                "-ft",
                "0"
           ])
           .arg(&self.id)
           .output_ok()
           .context("Failed to remove container")?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "Id")]
    pub id          : String,
    #[serde(rename = "Annotations")]
    pub annotations : HashMap<String, String>,
}

impl Image {
    pub fn from_id(id: &str) -> Result<Self> {
        let raw_json = Command::new("podman")
            .args([
                "inspect",
                "--type",
                "image",
                "--format",
                "json",
                id
            ])
            .output_ok()
            .context("Failed to inspect image JSON")?;

        // Podman is weird and always returns the JSON in an array, even when there can only be one element.
        let mut raw: Vec<Self> = serde_json::from_str(&raw_json)
                .context("Failed to deserialize image JSON")?;

        let out = raw
                .pop()
                .expect("Container JSON should always have at least one element");
        
        Ok(out)
    }

    pub fn enumerate() -> Result<Images> {
        let o = Command::new("podman")
            .args([
                "image",
                "ls",
                "--format",
                "{{.ID}}",
                "--filter",
                "dangling=false"
            ])
            .output_ok()
            .context("Failed to enumerate all image IDs")?;

        let mut out = vec![];
            
        for image in o
            .lines()
            .map(Image::from_id) 
        {   
            let image = image?;

            if let Some("box") = image.annotation("manager") {
                debug!(
                    "Enumerated image {}",
                    image.id
                );

                out.push(image);
            }
        }
        
        Ok(out)
    }

    pub fn instantiate(&self, replace: bool) -> Result<()> {
        let name = self.annotation("box.name")
            .expect("Name annotation should be set");

        let hash = self.annotation("box.hash")
            .expect("Hash annotation should be set");

        let mut args = vec![];

        for a in ANNOTATIONS {
            let key = format!("box.{a}");
            
            let Some(value) = self.annotation(&key) else {
                continue
            };

            if a == "args" {
                for v in value.split('\x1F') {
                    args.push(v.to_owned())
                }
            } else {
                let flag = format!("--{a}");

                for v in value.split('\x1F') {
                    args.push(flag.clone());
                    args.push(v.to_owned());
                }
            }
        }

        if replace {
            args.push(
                "--replace".to_owned()
            )
        }

        Command::new("podman")
            .arg("run")
            .arg("-d")
            .args([
                "--name",
                name,
                "--hostname",
                name,
            ])
            .args(args)
            .args([
                "--annotation",
                "manager=box"
            ])
            .arg("--annotation")
            .arg(format!("box.name={name}"))
            .arg("--annotation")
            .arg(format!("box.hash={hash}"))
            .arg(name)
            .spawn_ok()
            .context("Fault when instantiating image")?;
        
        Ok(())
    }

    pub fn annotation(&self, key: &str) -> Option<&str> {
        self
            .annotations
            .get(key)
            .map(String::as_str)
    }
}

pub fn push_annotation(ctr: &str, key: &str, data: &str) -> Result<()> {
    let format_str = format!(
        "{{{{index .ImageAnnotations \"{}\"}}}}",
        key
    );

    let old = Command::new("buildah")
        .arg("inspect")
        .arg("-t")
        .arg("container")
        .arg("--format")
        .arg(format_str)
        .arg(ctr)
        .output_ok()
        .context("Fault when retrieving annotation from working container")?;

    let old = old
        .split('\x1F')
        .chain([data])
        .collect();

    debug!("Writing {old:?} to {key} for {ctr}");

    write_annotation(ctr, key, old)
}

pub fn write_annotation(ctr: &str, key: &str, data: Vec<&str>) -> Result<()> {
    let mapping = format!(
        "{key}={}",
        data.join("\x1F").trim_start_matches('\x1F')
    );

    Command::new("buildah")
        .arg("config")
        .arg("-a")
        .arg(mapping)
        .arg(ctr)
        .spawn_ok()
        .context("Fault when writing annotation to working container")
}
