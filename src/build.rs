use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use crate::prelude::*;
use crate::podman::*;
use crate::CommandExt;

// For POSIX shells - build a script "on the fly" that sources the init using `source <(bx _init posix)`
// then executes the script in a subshell
// Something like:
//
// source <(bx _init posix)
// (
//    # Script contents
// )

pub type Definitions = Vec<Definition>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Definition {
    pub path: PathBuf,
    pub bang: String,
    pub hash: u64,
    pub tree: u64,
    pub meta: Metadata,
}

#[derive(Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub depends_on    : Vec<String>,
    #[serde(default)]
    pub containerfile : bool,
}

impl Definition {
    pub fn enumerate() -> Result<Definitions> {
        use std::fs;
        use std::ffi::OsStr;

        let dir = definition_directory()?;

        let mut out = vec![];

        for entry in fs::read_dir(dir).context("Fault when starting definition enumeration")? {
            let entry = entry
                .context("Fault when iterating over definition directory")
                .suggestion("Do you have permission issues?")?;

            if entry
                .file_type()
                .context("Failed to get entry file type")?
                .is_dir() 
            {
                continue;
            }

            if entry.path().extension() == OsStr::new("box").into() {
                out.push(
                    Definition::from_path(entry.path())
                )            
            }
        }

        let (defs, errors): (Vec<_>, Vec<_>) = out
            .into_iter()
            .partition(Result::is_ok);

        if !errors.is_empty() {
            let err = errors
                .into_iter()
                .map(Result::unwrap_err)
                .fold(eyre!("Failed to load and parse definition(s)"), |acc, err| {
                    acc.section(err)
                });

            Err(err)
        }
        else {
            let defs = defs
                .into_iter()
                .map(Result::unwrap)
                .collect();

            Ok(defs)
        }
    }

    pub fn find(name: &str) -> Result<Self> {
        use std::fs;
        use std::ffi::OsStr;

        let dir  = definition_directory()?;
        let stem = OsStr::new(name);

        let entry = fs::read_dir(dir)
            .context("Fault when starting definition search")?
            .filter_map(Result::ok)
            .find(|e| e.path().file_stem() == stem.into());

        if let Some(entry) = entry {
            Self::from_path(entry.path())
                .context("Failed to load and parse definition")
        }
        else {
            let suggestion = match Self::alternative(name) {
                Some(m) => format!("Did you mean '{}'?", m),
                None => "Did you make a typo?".to_string(),
            };

            let err = eyre!("Tried to operate on a definition ({name}) that does not exist")
                .suggestion(suggestion);

            Err(err)
        }
    }

    pub fn exists(name: &str) -> Result<bool> {
        use std::fs;

        let path = definition_directory()?
            .join(
                format!("{name}.box")
            );
        
        fs::exists(path)
            .map_err(|e| {
                Report::new(e)
                    .wrap_err(
                        format!("Fault when checking if definition ({name}) exists")
                    )
            })
    }

    pub fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        use std::fs;

        let path = p.as_ref().to_owned();

        let data = fs::read_to_string(&path)
            .context("Failed to read in definition data")
            .suggestion("Do you have permission issues or non-UTF-8 data?")?;

        let bang = data 
            .lines()
            .next()
            .context("Encountered an empty definition")?
            .to_owned();

        let meta = data
            .lines()
            .filter(|l| l.starts_with("#~"))
            .fold(String::new(), |mut acc, line| {
                acc += line.trim_start_matches("#~").trim();
                acc += "\n";
                acc
            });

        let meta: Metadata = toml::from_str(&meta)
            .context("Failed to deserialize TOML frontmatter")
            .suggestion("Did you make a typo?")?;

        
        let hash = seahash::hash(
            data.as_bytes()
        );

        let tree = hash;
        
        Ok(Self { path, bang, hash, tree, meta })
    }

    pub fn name(&self) -> &str {
        use std::ffi::OsStr;

        self
            .path
            .file_stem()
            .and_then(OsStr::to_str)
            .expect("Definition name should be valid UTF-8")
    }

    pub fn depends_on(&self) -> &[String] {
        &self.meta.depends_on
    }

    pub fn build(&self) -> Result<()> {
        use std::fs;
        use colored::Colorize;

        info!(
            "Building definition at path {:?}...",
            &self.path
        );

        eprintln!(
            "{} {}{}",
            "Building definition".bold().bright_white(),
            self.name().bold().green(),
            "...".bold().bright_white()
        );

        if self.meta.containerfile {
            self.build_containerfile()?;
        }
        else if self.bang.contains("fish") {
            Command::new("fish")
                .arg("-C")
                .arg("bx init fish | source")
                .arg(&self.path)
                .env(
                    "__BOX_BUILD_PATH",
                    &self.path
                )
                .env(
                    "__BOX_BUILD_DIR",
                    {
                        let mut p = self.path.to_owned();
                        p.pop();
                        p
                    }
                )
                .env(
                    "__BOX_BUILD_HASH",
                    format!("{:x}", self.hash)
                )
                .env(
                    "__BOX_BUILD_TREE",
                    format!("{:x}", self.tree)
                )
                .env(
                    "__BOX_BUILD_NAME",
                    self.name()
                )
                .spawn_ok()
                .context("Fault when evaluating Fish-based definition")?;
        }
        else {
            let script = format!(
                "source <(bx init posix)\n(\n{}\n)",
                fs::read_to_string(&self.path)
                    .context("Fault when reading in POSIX-based definition")?
            );

            Command::new("sh")
                .arg("-c")
                .arg(script)
                .env(
                    "__BOX_BUILD_PATH",
                    &self.path
                )
                .env(
                    "__BOX_BUILD_DIR",
                    {
                        let mut p = self.path.to_owned();
                        p.pop();
                        p
                    }
                )
                .env(
                    "__BOX_BUILD_HASH",
                    format!("{:x}", self.hash)
                )
                .env(
                    "__BOX_BUILD_TREE",
                    format!("{:x}", self.tree)
                )
                .env(
                    "__BOX_BUILD_NAME",
                    self.name()
                )
                .spawn_ok()
                .context("Fault when evaluating POSIX-based definition")?;
        }
        
        Ok(())
    }

    fn build_containerfile(&self) -> Result<()> {
        let path = format!(
            "box.path={}",
            self.path.to_string_lossy()
        );

        let name = format!(
            "box.name={}",
            self.name()
        );

        let hash = format!(
            "box.hash={:x}",
            self.hash
        );

        let tree = format!(
            "box.tree={:x}",
            self.tree
        );

        Command::new("podman")
            .args([
                "build",
                "--pull=newer",
                "--annotation", "manager=box",
            ])
            .arg("--annotation")
            .arg(&path)
            .arg("--annotation")
            .arg(&name)
            .arg("--annotation")
            .arg(hash)
            .arg("--annotation")
            .arg(tree)
            .arg("--tag")
            .arg(self.name())
            .arg("--file")
            .arg(&self.path)
            .spawn_ok()
            .context("Fault when evaluating Containerfile-based definition")?;

        Ok(())
    }

    /// Finds an alternative definition name that is similar to the given name.
    ///
    /// This function uses fuzzy matching to find a definition name that is close to the given name.
    /// If no match is found, it returns `None`.
    pub fn alternative(name: &str) -> Option<String> {
        use std::ffi::OsStr;

        use nucleo_matcher::{Matcher, Config};
        use nucleo_matcher::pattern::*;

        let defs = match Self::enumerate() {
            Ok(defs) => defs,
            Err(err) => {
                warn!("Failed to enumerate definitions for fuzzy matching: {}", err);
                return None;
            }
        };

        let names: Vec<_> = defs
            .iter()
            .filter_map(|d| {
                d
                    .path
                    .file_stem()
                    .and_then(OsStr::to_str)
            })
            .collect();

        let mut matcher = Matcher::new(Config::DEFAULT);

        Pattern::new(
            name,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy
        )
        .match_list(names, &mut matcher)
        .first()
        .map(|(m, _)| m)
        .copied()
        .map(str::to_owned)
    }
}

impl Definition {
    pub fn create(name: String) -> Result<()> {
        use std::fs::File;
        use dialoguer::Editor;

        if Self::exists(&name)? {
            let err = eyre!("Definition {name} already exists")
                .suggestion("You may want to edit or delete it instead.");

            return Err(err);
        }

        let path = definition_directory()?
            .join(
                format!("{name}.box")
            );

        File::create(&path)
            .context("Fault when creating definition file")?;

        if let Some(data) = Editor::new()
            .require_save(true)
            .edit("#!/bin/bash\n\n")
            .context("Fault when editing new definition")?
        {
            std::fs::write(&path, data)
                .context("Fault when writing new definition to file")?
        }
        else {
            warn!("Definition creation aborted!");

            std::fs::remove_file(&path)
                .context("Fault when removing unwanted definition file")?;

            bail!("Definition creation aborted")
        }
        
        Ok(())
    }

    pub fn edit(name: String) -> Result<()> {
        use dialoguer::Editor;

        if !Self::exists(&name)? {
            let err = eyre!("Definition {name} does not exist")
                .note("Box checked the path {path:?}")
                .suggestion("Maybe create it first?");

            return Err(err);
        }

        let path = definition_directory()?
            .join(
                format!("{name}.box")
            );

        let data = std::fs::read_to_string(&path)
            .context("Fault when reading in definition data for editing")?;

        if let Some(data) = Editor::new()
            .require_save(true)
            .edit(&data)
            .context("Fault when editing definition")?
        {
            std::fs::write(&path, data)
                .context("Fault when writing definition to file")?
        }
        else {
            warn!("Definition edit aborted!");
            bail!("Definition edit aborted")
        }

        Ok(())
    }

    pub fn delete(name: String, yes: bool) -> Result<()> {
        use dialoguer::Confirm;

        if !Self::exists(&name)? {
            let err = eyre!("Definition {name} does not exist")
                .note("Box checked the path {path:?}")
                .suggestion("Maybe create it first?");

            return Err(err);
        }

        let path = definition_directory()?
            .join(
                format!("{name}.box")
            );

        if !yes {
            let confirm = Confirm::new()
                .with_prompt(
                    format!("Are you sure you want to remove the definition {name:?}")
                )
                .interact()
                .context("Fault when asking for user confirmation")?;

            if !confirm {
                return Ok(())
            }
        }

        std::fs::remove_file(path)
            .context("Fault when removing definition")?;

        Ok(())
    }
}

pub fn definition_directory() -> Result<PathBuf> {
    let options = || {
        if let Ok(dir) = std::env::var("BOX_DEFINITION_DIR") {
            return Some(
                PathBuf::from(dir)
            );
        }
    
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            return Some(
                PathBuf::from(xdg_config)
                    .join("box")
            );
        }
    
        if let Ok(home) = std::env::var("HOME") {
            return Some(
                PathBuf::from(home)
                    .join(".config")
                    .join("box")
            );
        }

        None
    };

    match options() {
        Some(dir) => {
            if !dir.exists() {
                std::fs::create_dir_all(&dir)
                    .context("Failed to create definition directory")?;
            }
            
            Ok(dir)
        },
        None => {
            let err = eyre!("Could not find a valid directory for definitions")
                .note("Box needs a place to store container definitions.")
                .suggestion("You likely have something wrong with your environment; Box tries:\n\t* $BOX_DEFINITION_DIR\n\t* $XDG_CONFIG_HOME/box\n\t* $HOME/.config/box\n... in that order.");

            Err(err)
        }
    }
}

pub fn build_set(defs: &[String], all: bool, force: bool) -> Result<()> {   
    use petgraph::Graph;
    use petgraph::algo::toposort;
    use petgraph::visit::Dfs;

    let mut set: Vec<_> = match all {
        false => {
            let (defs, errors): (Vec<_>, Vec<_>) = defs
                .iter()
                .map(String::as_ref)
                .map(Definition::find)
                .partition(Result::is_ok);
            
            if !errors.is_empty() {
                let err = errors
                    .into_iter()
                    .map(Result::unwrap_err)
                    .fold(eyre!("Failed to load and parse definition(s)"), |acc, err| {
                        acc.section(err)
                    });
    
                return Err(err)
            }
            else {
                defs
                    .into_iter()
                    .map(Result::unwrap)
                    .collect()
            }
        },
        true => Definition::enumerate()?
    };

    if set.is_empty() {
        let err = eyre!("No definitions found")
            .suggestion("Did you forget to provide the definition(s) to operate on?")
            .suggestion("Alternatively, if you meant to build all definiitions, pass the -a/--all flag.");

        return Err(err);
    }

    debug!(
        "Finished build set enumeration - got {} (all: {all})\n{set:#?}",
        set.len()
    );

    debug!("Resolving dependencies...");
    
    let mut names: HashSet<_> = set
        .iter()
        .map(Definition::name)
        .collect();

    let mut deps = vec![];

    for name in set
        .iter()
        .flat_map(Definition::depends_on)
        .map(String::as_str)
    {
        if names.contains(name) {
            continue;
        }
        
        let def = Definition::find(name)
            .context("Fault when searching for definition dependency")?;

        debug!(
            "Fetched dependency {:?}",
            def
        );

        deps.push(def);
        names.insert(name);
    }

    set.extend(deps);

    debug!(
        "Finished fetching dependencies - now working with {}\n{set:#?}",
        set.len()
    );

    let mut indices = HashMap::new();
    let mut graph   = Graph::<Definition, ()>::new();

    for def in set {
        indices.insert(
            def.name().to_owned(),
            graph.add_node(def)
        );
    }

    for idx in graph.node_indices() {
        // Borrow check complains about an immutable borrow
        // on the graph if we don't clone the dependencies.
        #[allow(clippy::unnecessary_to_owned)]
        for dep in graph[idx].depends_on().to_vec() {
            // We (counter-intuitively, at least to me)
            // insert edges in reverse; otherwise, the final
            // topological sort is inverted.
            graph.update_edge(
                indices[&dep],
                idx,
                ()
            );
        }
    }

    debug!("Walking set graph to compute tree hashes for each definition...");

    // We reverse the graph temporarily
    // in order to make the DFS work.
    graph.reverse();

    for idx in graph.node_indices() {
        debug!("Walking from {:?}", graph[idx]);

        let mut search = Dfs::new(&graph, idx);

        while let Some(nx) = search.next(&graph) {
            debug!("{:?} -> {:?}", graph[idx], graph[nx]);

            if graph[idx].tree != graph[nx].hash {
                // While probably not cryptographically sound,
                // XORing hashes together like this is commutative.
                graph[idx].tree ^= graph[nx].hash;
            }
        }
    }

    graph.reverse();

    debug!("Topologically sorting build set...");

    let topo = toposort(&graph, None)
        .map_err(|e| eyre!{"{e:?}"})
        .context("Cycle detected in definition dependency graph")?;
        
    if force {
        for idx in topo {
            graph[idx].build()?;
        }

        debug!("Finished building definition set!");

        return Ok(());
    }

    let to_u64 = |s| u64::from_str_radix(s, 16)
        .expect("Hash annotation should be a 64-bit hexadecimal number");

    let path_hash: HashMap<_, _> = Image::enumerate()
        .context("Fault when enumerating images for change detection")?
        .iter()
        .map(|i| 
            (
                i.annotation("box.path")
                    .map(PathBuf::from)
                    .expect("Path annotation should be set"),
                (
                    i.annotation("box.hash")
                        .map(to_u64)
                        .expect("Hash annotation should be set"),
                    i.annotation("box.tree")
                        .map(to_u64)
                        .expect("Tree hash annotation should be set")
                )
            )
        )
        .collect();

    debug!("Path -> Hash mapping computed:\n{path_hash:?}");

    for idx in topo {
        let def = &graph[idx];

        debug!("Inspecting... {def:?}");

        // If no image with a corresponding path exists, build.
        let Some(hashes) = path_hash.get(&def.path) else {
            def.build()?;
            continue
        };

        debug!("Hashes: {hashes:?}");

        let (own, tree) = hashes;
        
        if *own != def.hash || *tree != def.tree {
            def.build()?;
        }
    }

    Ok(())
}