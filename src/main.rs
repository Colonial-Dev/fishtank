mod build;
mod cli;
mod podman;

mod prelude {
    pub use color_eyre::eyre::{
        eyre,
        Context as EyreContext,
        ContextCompat
    };
    
    pub use color_eyre::{
        Result,
        Report,
        Section,
        SectionExt,
    };

    pub use log::{
        debug,
        info,
        warn,
        error
    };
}

use prelude::*;
use build::*;
use cli::*;
use podman::*;

pub const VERSION    : &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS    : &str = env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY : &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> Result<()> {
    use clap::Parser;
    use Command::*;

    let args = Cli::parse();

    // Only use the verbosity flag if RUST_LOG isn't already set.
    if std::env::var("RUST_LOG").is_err() {
        match args.verbose {
            0 => std::env::set_var("RUST_LOG", "none"),
            1 => std::env::set_var("RUST_LOG", "info"),
            2 => std::env::set_var("RUST_LOG", "debug"),
            _ => std::env::set_var("RUST_LOG", "trace"),
        }
    }

    install_logging();

    info!("Box v{VERSION} by {AUTHORS}");
    info!("This program is licensed under the GNU Affero General Public License, version 3.");
    info!("See {REPOSITORY} for more information.");
    info!("Parsed arguments: {args:#?}");

    ensure("podman")?;
    ensure("buildah")?;

    let retrieve_set = |set: &ContainerSet| match set.all {
        false => {
            let mut out = vec![];

            for id in &set.containers {
                existence_check(id)?;

                out.push(
                    Container::from_id(id)?
                )
            }
            
            Ok(out)
        },
        true  => Container::enumerate(),
    };

    match args.command {
        Containers => list_containers()?,
        Images     => list_images()?,

        Create { name } => Definition::create(name)?,
        Edit   { name } => Definition::edit(name)?,
        Delete { name } => Definition::delete(name)?,

        // Enter
        // Exec
        // Ephemeral

        Build { defs, all, force } => build_set(&defs, all, force)?,


        Start   (set) => retrieve_set(&set)?.iter().try_for_each(Container::start)?,
        Stop    (set) => retrieve_set(&set)?.iter().try_for_each(Container::stop)?,
        Restart (set) => retrieve_set(&set)?.iter().try_for_each(Container::restart)?,
        Down    (set) => retrieve_set(&set)?.iter().try_for_each(Container::down)?,
        Reup    (set) => retrieve_set(&set)?.iter().try_for_each(Container::reup)?,

        // Up      (set) => retrieve_set(&set)?.iter().try_for_each(Container::up)?,


        Init { shell } => match &*shell {
            "fish"  => init_fish(),
            "posix" => init_posix(),
            _       => unreachable!()
        },
        Config { operation, rest } => match operation {
            _ => todo!()
        },
        _ => todo!()
    }

    Ok(())
}

fn install_logging() {
    env_logger::init();

    color_eyre::config::HookBuilder::new()
        .panic_section("Well, this is embarassing. It appears Box has crashed!\nConsider reporting the bug at <https://github.com/Colonial-Dev/box>.")
        .capture_span_trace_by_default(true)
        .display_location_section(false)
        .install()
        .expect("Could not install Eyre hooks!");
}

/// Checks if a container exists, returning a well-formed error (with fuzzy-matched suggestions) if not.
fn existence_check(id: &str) -> Result<()> {
    use nucleo_matcher::{Matcher, Config};
    use nucleo_matcher::pattern::*;

    if Container::exists(id)? {
        Ok(())
    }
    else {
        let containers = Container::enumerate()
            .context("Failed to enumerate containers for fuzzy matching")?;

        let names = containers
            .iter()
            .filter_map(|c| c.annotation("box.name") );

        let mut matcher = Matcher::new(Config::DEFAULT);

        let matches = Pattern::new(
            id,
            CaseMatching::Ignore,
            Normalization::Smart,
            AtomKind::Fuzzy
        ).match_list(names, &mut matcher);

        let suggestion = match matches.first() {
            Some(m) => format!("Did you mean: {}", m.0),
            None => "Did you make a typo?".to_string(),
        };

        let err = eyre!("Tried to operate on a container ({id}) that does not exist")
            .suggestion(suggestion);

        Err(err)
    }
}

/// Checks that a program exists on the system's PATH, returning a well-formed error if not.
fn ensure(program: &str) -> Result<()> {
    use std::io::ErrorKind;
    use std::process::Command;

    match Command::new(program)
        .arg("--version")
        .output()
    {
        Ok(out) => {
            info!(
                "Ensured {}",
                String::from_utf8_lossy(&out.stdout)
            );

            Ok(())
        },
        Err(e) => {
            let err = if let ErrorKind::NotFound = e.kind() {
                eyre!("{program} not found in PATH")
                    .note("Box needs this program to run.")
                    .suggestion("Make sure it's installed and in your PATH.")
            } else {
                eyre!(e)
                    .wrap_err("Unable to determine if {program} exists")
                    .note("Box needs this program to run.")
                    .suggestion("Maybe you have permission issues?")
            };

            Err(err)
        }
    }
}

fn list_containers() -> Result<()> {
    Ok(())
}

fn list_images() -> Result<()> {
    Ok(())
}

fn init_posix() {
    //print!(
    //    include_str!("shell/posix.sh")
    //)
}

fn init_fish() {
    //print!(
    //    include_str!("shell/fish.sh")
    //)
}

pub trait CommandExt {
    /// Extension method.
    /// 
    /// Wraps `output` to return either a (lossy) UTF-8 string of standard output _or_ a well-formatted error.
    fn output_ok(&mut self) -> Result<String>;
}

impl CommandExt for std::process::Command {
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