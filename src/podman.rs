use std::collections::HashMap;
use std::process::Command;

use serde::Deserialize;

use crate::prelude::*;

pub type Containers = Vec<Container>;
pub type Images     = Vec<Image>;

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
            #[serde(rename = "Image")]
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

    pub fn remove(&self) -> Result<()> {
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
    pub id          : String,
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

    pub fn exists(id: &str) -> Result<bool> {
        let output = Command::new("podman")
            .args([
                "image",
                "exists",
                id
            ])
            .output()
            .context("Failed to check if image exists")?;

        Ok(
            output.status.success()
        )
    }

    pub fn annotation(&self, key: &str) -> Option<&str> {
        self
            .annotations
            .get(key)
            .map(String::as_str)
    }
}

// TODO Create new containers from managed images.

trait CommandExt {
    /// Extension method.
    /// 
    /// Wraps `output` to return either a (lossy) UTF-8 string of standard output _or_ a well-formatted error.
    fn output_ok(&mut self) -> Result<String>;
}

impl CommandExt for Command {
    fn output_ok(&mut self) -> Result<String> {
        debug!("Shelling out; command is {self:?}");
        
        let o = self.output()?;
    
        if o.status.success() {
            let stdout = String::from_utf8_lossy(&o.stdout)
                .to_string();
            
            Ok(stdout)
        }
        else {
            error!("Command invocation failed!");

            let arguments = format!(
                "{:?} {:?}",
                self.get_program(),
                self.get_args()
            ).header("Arguments:");
    
            let stderr = String::from_utf8_lossy(&o.stderr)
                .to_string()
                .header("Standard error:");
    
            let stdout = String::from_utf8_lossy(&o.stdout)
                .to_string()
                .header("Standard output:");
    
            let err = eyre!("command invocation failed")
                .section(arguments)
                .section(stderr)
                .section(stdout)
                .note("This is likely due to invalid input or a bug in Box.");
    
            Err(err)
        }
    }
}
