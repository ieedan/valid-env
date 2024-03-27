use crate::CONFIG_PATH;
use colored::Colorize;
use serde_json;
use std::{collections::HashMap, fs};
use vnv::{
    parsing::config,
    util::{ask_yes_no, request_value, Answer},
};

const INIT_MESSAGE: &str = r#"                  
__   ___ ____   __
\ \ / / '_ \ \ / /
 \ V /| | | \ V / 
  \_/ |_| |_|\_/  
                  
"#;

const DEFAULT_SRC: &str = r#"@matches("bar")
FOO="bar"
"#;

const DEFAULT_GIT_IGNORE: &str = r#".vnv
.env"#;

/// Initializes the config file and optionally a template file
pub fn default() {
    match fs::read(CONFIG_PATH) {
        Ok(_) => {
            println!("{} vnv already initialized.", "Error:".bold().red());
            return;
        }
        _ => {}
    };

    println!("{INIT_MESSAGE}");

    let mut config = config::Options::new();

    request_value(&mut config.src, "Where is the source file?");

    let result = fs::read(&config.src);

    if result.is_ok() {
        match ask_yes_no("Overwrite source file", Answer::No) {
            Answer::Yes => {
                println!("Overwriting source file at {}", config.src);

                fs::write(&config.src, DEFAULT_SRC).unwrap();
            }
            Answer::No => {}
        }
    } else {
        println!("Creating source file at {}", config.src);

        fs::write(&config.src, DEFAULT_SRC).unwrap();
    }

    request_value(&mut config.build.output, "Where to write the build output?");

    match ask_yes_no("Hide environment variables in std out", Answer::No) {
        Answer::Yes => config.cloak = true,
        Answer::No => config.cloak = false,
    }

    match ask_yes_no("Keep comments and decorator comments in .env", Answer::Yes) {
        Answer::Yes => config.build.minify = false,
        Answer::No => config.build.minify = true,
    }

    match fs::read(".gitignore") {
        Ok(content) => {
            let mut content = String::from_utf8(content).unwrap();
            let lines = content.split("\n");

            let needs_ignore = vec![".env", ".vnv"];

            let mut ignore_map = HashMap::new();

            for file in needs_ignore {
                ignore_map.insert(file, 0);
            }

            for line in lines {
                if ignore_map.contains_key(line.trim()) {
                    ignore_map.remove(line.trim());
                }
            }

            for (k, _) in ignore_map {
                content = content.trim().to_owned();

                content.push_str(&format!("\n{k}"));

                println!("Adding {k} to .gitignore...");
            }

            fs::write(".gitignore", content).unwrap();
        }
        Err(_) => {
            println!("Creating .gitignore...");

            fs::write(".gitignore", DEFAULT_GIT_IGNORE).unwrap();
        }
    }

    let config_content = serde_json::to_string_pretty(&config).unwrap();

    println!("Writing preferences to {CONFIG_PATH}.");

    fs::write(&CONFIG_PATH, config_content).unwrap();
}
