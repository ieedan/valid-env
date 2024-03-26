use clap::Parser;
use vnv::parsing::config;
mod commands;

use commands::{build, check, template, Commands};

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
        Commands::Check { file, cloak } => {
            // Overrides config with passed arguments
            if let Some(file) = file {
                config.src = file;
            }

            if cloak {
                config.cloak = true;
            }

            let options = check::Options { default: config };
            commands::check(options);
        }
        Commands::Build {} => {
            let options = build::Options { default: config };
            commands::build(options);
        }
        Commands::Init {} => commands::init(),
        Commands::Template { yes } => {
            let options = template::Options { default: config, yes };
            commands::template(options);
        }
    }
}
