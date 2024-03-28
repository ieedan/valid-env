use clap::Parser;
use colored::Colorize;
use vnv::parsing::{config, Environment};
mod commands;

use commands::{build, check, Commands};

#[derive(Parser, Debug)]
#[command(version, about, author, long_about = None)]
struct Cli {
    /// Commands to execute
    #[clap(subcommand)]
    command: Commands,
}

pub const CONFIG_PATH: &str = ".vnv.config.json";

fn main() {
    let args = Cli::parse();

    let mut config = config::parse(CONFIG_PATH);

    let mut environment: Environment = Environment::Dev;

    match args.command {
        Commands::Check {
            file,
            cloak,
            dev,
            prod
        } => {
            // Overrides config with passed arguments
            if let Some(file) = file {
                config.src = file;
            }

            if cloak {
                config.cloak = true;
            }

            if dev && prod {
                println!("{} You provided multiple environment flags (--dev, --prod) defaulting to the development environment", "WARN:".bold().bright_yellow())
            } else if prod {
                environment = Environment::Prod;
            }

            let options = check::Options { config, environment };
            commands::check(options);
        }
        Commands::Build { dev, prod } => {
            if dev && prod {
                println!("{} You provided multiple environment flags (--dev, --prod) defaulting to the development environment", "WARN:".bold().bright_yellow())
            } else if prod {
                environment = Environment::Prod;
            }
            
            let options = build::Options { config, environment };
            commands::build(options);
        }
        Commands::Init {} => commands::init(),
    }
}
