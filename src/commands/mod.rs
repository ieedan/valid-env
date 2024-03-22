use clap::Subcommand;

pub mod check;

pub use crate::check::default as check;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate the .vnv file
    Check(check::Options),
}