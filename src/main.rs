use clap::Parser;
use commands::{build, init, Commands};
use vnv::parsing::config;

use crate::commands::check;

mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = ".vnv.config.json")]
    config: String,

    /// Path of file to validate, defaults to ".vnv" if not specified.
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Will hide the values in the ".vnv" from being output to the std out
    #[clap(short, long)]
    cloak: Option<bool>,

    /// Commands to execute
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();

    let mut config = config::parse(&args.config);

    // Overrides config with passed arguments
    if let Some(file) = args.file {
        config.src = file;
    }

    if let Some(cloak) = args.cloak {
        config.cloak = cloak;
    }

    match args.command {
        Commands::Check { } => {
            let options = check::Options {
                default: config
            };
            commands::check(options);
        },
        Commands::Build { } => {
            let options = build::Options {
                default: config
            };
            commands::build(options);
        },
        Commands::Init { } => {
            commands::init();
        },
    }
}
