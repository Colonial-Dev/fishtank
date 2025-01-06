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
        Section,
        SectionExt
    };

    pub use log::{
        trace,
        debug,
        info,
        warn,
        error
    };
}

use prelude::*;
use cli::*;
use podman::*;

pub const VERSION    : &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS    : &str = env!("CARGO_PKG_AUTHORS");
pub const REPOSITORY : &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> Result<()> {
    use clap::Parser;

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

    // match args.command

    Ok(())
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

fn install_logging() {
    env_logger::init();

    color_eyre::config::HookBuilder::new()
        .panic_section("Well, this is embarassing. It appears Box has crashed!\nConsider reporting the bug at <https://github.com/Colonial-Dev/box>.")
        .capture_span_trace_by_default(true)
        .display_location_section(false)
        .install()
        .expect("Could not install Eyre hooks!");
}