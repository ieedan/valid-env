use clap::Subcommand;

pub mod check;

pub use check::default as check;

pub mod build;

pub use build::default as build;

pub mod init;

pub use init::default as init;

pub mod template;

pub use template::default as template;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate the .vnv file
    Check {
        /// Path of file to validate, defaults to ".vnv" if not specified.
        #[clap(short, long, value_parser)]
        file: Option<String>,

        /// Will hide the values in the ".vnv" in the std out
        #[clap(short, long)]
        cloak: Option<bool>,
    },
    /// Convert the .vnv file to a valid .env file
    Build {},
    /// Initializes .vnv by creating the source file and settings file as well as configuring your .gitignore
    Init {},
    /// Generates a template file from the current .vnv file
    Template { }
}
