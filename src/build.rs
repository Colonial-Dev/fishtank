use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use walkdir::WalkDir;

use crate::prelude::*;

// metadata.is_file() && permissions.mode() & 0o111 != 0

// For POSIX shells - build a script "on the fly" that sources the init using `source <(bx _init posix)`
// then executes the script in a subshell

pub type Definitions = Vec<Definition>;

#[derive(Debug)]
pub struct Definition {
    pub path: PathBuf,
    pub hash: u64,
    pub meta: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub depends_on    : Vec<String>,
    #[serde(default)]
    pub containerfile : bool,
}

impl Definition {
    pub fn enumerate() -> Result<Definitions> {
        use std::ffi::OsStr;
        use walkdir::WalkDir;

        let dir = definition_directory()?;

        let mut out = vec![];

        for entry in WalkDir::new(dir) {
            let entry = entry
                .context("Fault when iterating over definition directory")
                .suggestion("Do you have permission issues?")?;

            if entry.file_type().is_dir() {
                continue;
            }

            if entry.path().extension() == OsStr::new("box").into() {
                continue;
            }

            out.push(
                Definition::from_path(entry.path())
            )
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
        use std::ffi::OsStr;
        
        use nucleo_matcher::{Matcher, Config};
        use nucleo_matcher::pattern::*;

        let dir  = definition_directory()?;
        let stem = OsStr::new(name);

        let entry = WalkDir::new(dir)
            .into_iter()
            .filter_map(Result::ok)
            .find(|e| e.path().file_stem() == stem.into());

        if let Some(entry) = entry {
            Self::from_path(entry.path())
                .context("Failed to load and parse definition")
        }
        else {
            let defs = Self::enumerate()
                .context("Failed to enumerate definitions for fuzzy matching")?;

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

            let matches = Pattern::new(
                name,
                CaseMatching::Ignore,
                Normalization::Smart,
                AtomKind::Fuzzy
            ).match_list(names, &mut matcher);
    
            let suggestion = match matches.first() {
                Some(m) => format!("Did you mean: {}", m.0),
                None => "Did you make a typo?".to_string(),
            };

            let err = eyre!("Tried to operate on a definition ({name}) that does not exist")
                .suggestion(suggestion);

            Err(err)
        }
        
    }

    pub fn from_path(p: impl AsRef<Path>) -> Result<Self> {
        let path = p.as_ref().to_owned();

        let data = fs::read_to_string(&path)
            .context("Failed to read in definition data")
            .suggestion("Do you have permission issues or non-UTF-8 data?")?;

        let meta = data
            .lines()
            .filter(|l| l.starts_with("#~"))
            .fold(String::new(), |mut acc, line| {
                acc += line;
                acc += "\n";
                acc
            });

        let meta: Metadata = toml::from_str(&meta)
            .context("Failed to deserialize TOML frontmatter")
            .suggestion("Did you make a typo?")?;

        
        let hash = seahash::hash(
            data.as_bytes()
        );
        
        Ok(Self { path, hash, meta })
    }

    pub fn build(&self) -> Result<()> {
        info!(
            "Building definition at path {:?}...",
            &self.path
        );

        // Branch on Fish | POSIX | Containerfile
        // - IF metadata has 'containerfile = true' then branch
        // - EIF metadata has a shebang containing 'fish' then branch
        // - E assume POSIX

        info!(
            "Finished building definition at path {:?}",
            &self.path
        );

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
    let set: Vec<_> = match all {
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
        true  => Definition::enumerate()?
    };

    if set.is_empty() {
        let err = eyre!("No definitions found")
            .suggestion("Did you forget to provide the definition(s) to operate on?")
            .suggestion("Alternatively, if you meant to build all containers, pass the -a/--all and/or -f/--force flags.");

        return Err(err);
    }

    let set: Vec<_> = match force {
        false => {
            // Filter unchanged
            // Error if no changed
            todo!()
        },
        true  => set
    };
    
    // Screen for duplicates

    // Also needed is a dependency resolution algorithm.
    // Given a graph containing an arbitrary number of disjoint subgraphs (each of which may have 1 or more members)
    // we must:
    // - Identify and extract all disjoint subgraphs (repeated D/BFS should do the trick)
    // - Ensure that all multi-member subgraphs are acyclic (and have no "broken edges")
    // - Convert all subgraphs into queues that respect encoded dependency relationships
    // Canonical algorithm: topological sort
    //
    // Also note that when working in non-force contexts, we _must_ handle the case
    // where we have an _unchanged_ definition with _changed_ dependencies.
    // Possible solution: hash XOR of all dependencies.

    // Perform builds
    Ok(())
}