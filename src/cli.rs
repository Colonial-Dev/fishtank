use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Controls logging level.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
pub struct ContainerSet {
    /// One or more containers; can use names and IDs interchangeably.
    pub containers: Vec<String>,
    /// Whether or not to operate on *all* containers.
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compile definitions into container images.
    Build {
        /// The definitions to build.
        defs: Vec<String>,
        /// Whether or not to operate on all definitions.
        #[arg(short, long)]
        all: bool,
        /// Whether or not to ignore unchanged definitions.
        #[arg(short, long)]
        force: bool,
    },
    /// List all managed containers (alias: ls)
    #[clap(alias = "ls")]
    Containers,
    /// Create a new container definition.
    Create { name: String },
    /// List all managed definitions (alias: lsd)
    #[clap(alias = "lsd")]
    Definitions,
    /// Delete a container definition.
    Delete { name: String, #[arg(short, long)] yes: bool },
    /// Remove managed container(s).
    Down    (ContainerSet),
    /// Edit an existing container definition.
    Edit   { name: String },
    /// Invoke $SHELL inside a container.
    Enter  { name: String },
    /// Execute a command inside a new ephemeral container.
    Ephemeral {
        /// The name or ID of the image to use.
        name: String,
        /// The program to execute.
        path: String,
        /// Arguments to the program, if any.
        #[arg(allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Execute a command inside a container.
    Exec {
        /// The name of the container.
        name: String,
        /// The program to execute.
        path: String,
        /// Arguments to the program, if any.
        #[arg(allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Restart managed container(s).
    Restart (ContainerSet),
    /// Remove and re-create managed container(s).
    Reup    (ContainerSet),
    /// Start managed container(s).
    Start   (ContainerSet),
    /// Stop managed containers(s).
    Stop    (ContainerSet),
    /// Create managed container(s).
    Up {
        /// One or more images; can use names and IDs interchangeably.
        containers: Vec<String>,
        /// Whether or not to operate on *all* images.
        #[arg(short, long)]
        all: bool,
        /// Whether or not to replace existing containers.
        #[arg(short, long)]
        replace: bool,
    },

    #[clap(hide = true)]
    Init {
        shell: String,
    },
    #[clap(hide = true)]
    Config {
        operation : String,
        #[arg(allow_hyphen_values = true)]
        args      : Vec<String>,
    }
}
