use clap::Subcommand;

pub mod check;

pub use check::default as check;

pub mod build;

pub use build::default as build;

pub mod init;

pub use init::default as init;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate the .vnv file
    Check {
        /// Path of file to validate, defaults to ".vnv" if not specified.
        #[clap(short, long, value_parser)]
        file: Option<String>,

        /// Will hide the values in the ".vnv" in the std out
        #[clap(short, long, action = clap::ArgAction::SetTrue)]
        cloak: bool,

        #[clap(short, long, action = clap::ArgAction::SetTrue)]
        dev: bool,
        #[clap(short, long, action = clap::ArgAction::SetTrue)]
        prod: bool,
    },
    /// Convert the .vnv file to a valid .env file
    Build {
        #[clap(short, long, action = clap::ArgAction::SetTrue)]
        dev: bool,
        #[clap(short, long, action = clap::ArgAction::SetTrue)]
        prod: bool,
    },
    /// Initializes .vnv by creating the source file and settings file as well as configuring your .gitignore
    Init {},
}
