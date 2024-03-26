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

        /// Will hide the values in the ".vnv" from being output to the std out
        #[clap(short, long)]
        cloak: Option<bool>,
    },
    Build {},
    Init {},
}
