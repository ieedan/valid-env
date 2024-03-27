use clap::Parser;
use vnv::parsing::config;
mod commands;

use commands::{build, check, Commands};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Commands to execute
    #[clap(subcommand)]
    command: Commands,
}

pub const CONFIG_PATH: &str = ".vnv.config.json";

fn main() {
    let args = Cli::parse();

    let mut config = config::parse(CONFIG_PATH);

    match args.command {
        Commands::Check { file, cloak, template } => {
            // Overrides config with passed arguments
            if let Some(file) = file {
                config.src = file;
            }

            if cloak {
                config.cloak = true;
            }

            let options = check::Options { config, template };
            commands::check(options);
        }
        Commands::Build {} => {
            let options = build::Options { config };
            commands::build(options);
        }
        Commands::Init {} => commands::init()
    }
}
