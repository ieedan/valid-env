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

    },
    Build {

    },
    Init {
        
    },
}
