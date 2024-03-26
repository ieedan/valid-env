use crate::CONFIG_PATH;
use colored::Colorize;
use serde_json;
use std::{collections::HashMap, fs};
use vnv::{
    parsing::config,
    util::{printf, ask_yes_no, request_value, Answer},
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

const DEFAULT_TEMPLATE: &str = r#"# This file should be committed to source control and serves as the blue-print for your .vnv file
# List the required variables here and their constraints while omitting their values
# vnv will check the .vnv file against this file to make sure that they match 

@matches("bar")
FOO
"#;

const DEFAULT_GIT_IGNORE: &str = r#".vnv
.env"#;

/// Initializes the config file and optionally a template file
pub fn default() {
    match fs::read(CONFIG_PATH) {
        Ok(_) => {
            println!("{} vnv already initialized.", "Error:".bold().red());
            return;
        },
        _ => {}
    };

    println!("{INIT_MESSAGE}");

    let mut config = config::Options::new();

    request_value(&mut config.src, "Where is the source file?");

    let result = fs::read(&config.src);

    let mut fresh_file = false;

    if result.is_ok() {
        match ask_yes_no("Overwrite source file", Answer::No) {
            Answer::Yes => {
                println!("Overwriting source file at {}", config.src);

                fs::write(&config.src, DEFAULT_SRC).unwrap();

                fresh_file = true;
            }
            Answer::No => {}
        }
    } else {
        println!("Creating source file at {}", config.src);

        fs::write(&config.src, DEFAULT_SRC).unwrap();

        fresh_file = true;
    }

    if fresh_file {
        match ask_yes_no("Use a template file", Answer::Yes) {
            Answer::Yes => {
                request_value(&mut config.template, "Where is the template file?");

                let result = fs::read(&config.template);

                if result.is_ok() {
                    printf(&format!(
                        " y/N? {}",
                        "N".truecolor(125, 125, 125)
                    ));

                    printf("\x1B[1D");

                    match ask_yes_no("Overwrite template file", Answer::No) {
                        Answer::Yes => {
                            println!("Overwriting template file at {}", config.template);

                            fs::write(&config.template, DEFAULT_TEMPLATE).unwrap();
                        }
                        Answer::No => {}
                    }
                } else {
                    println!("Creating template file at {}", config.template);

                    fs::write(&config.template, DEFAULT_TEMPLATE).unwrap();
                }
            }
            Answer::No => {}
        }
    }

    request_value(&mut config.build.output, "Where to write the build output?");

    match ask_yes_no("Hide environment variables in std out", Answer::No) {
        Answer::Yes => config.cloak = true,
        Answer::No => config.cloak = false
    }

    match ask_yes_no("Keep comments and decorator comments in .env", Answer::Yes) {
        Answer::Yes => config.build.minify = false,
        Answer::No => config.build.minify = true
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
