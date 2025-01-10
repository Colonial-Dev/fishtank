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
    /// List all managed containers (alias: ls)
    #[clap(alias = "ls")]
    Containers,
    /// List all managed images (alias: lsi)
    #[clap(alias = "lsi")]
    Images,

    /// Create a new container definition.
    Create { name: String },
    /// Edit an existing container definition.
    Edit   { name: String },
    /// Delete a container definition.
    Delete { name: String },
    /// Invoke $SHELL inside a container.
    Enter  { name: String },
    /// Execute a command inside a container.
    Exec {
        /// The name of the container.
        name: String,
        /// The program to execute.
        path: String,
        /// Arguments to the program, if any.
        args: Vec<String>,
    },
    /// Create a new container from the provided image and execute a command inside it;
    /// the container will be deleted after the command terminates.
    Ephemeral {
        /// The name of the image.
        name: String,
        /// The program to execute.
        path: String,
        /// Arguments to the program, if any.
        args: Vec<String>,
    },
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

    /// Start managed container(s).
    Start   (ContainerSet),
    /// Stop managed containers(s).
    Stop    (ContainerSet),
    /// Restart managed container(s).
    Restart (ContainerSet),
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
    /// Remove managed container(s).
    Down    (ContainerSet),
    /// Remove and re-create managed container(s).
    Reup    (ContainerSet),

    #[clap(hide = true)]
    Init {
        shell: String,
    },
    #[clap(hide = true)]
    Config {
        operation : String,
        rest      : Vec<String>, 
    }
}
