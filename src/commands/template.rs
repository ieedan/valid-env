use colored::Colorize;
use std::fs;
use vnv::decorators::DecoratorValue;
use vnv::util::{ask_yes_no, Answer};

use vnv::parsing::{self, config};

use crate::{CONFIG_PATH, DEFAULT_TEMPLATE_PATH};

pub struct Options {
    pub config: config::Options,
    pub yes: bool,
}

pub fn default(options: Options) {
    let no_configured_path = options.config.template.is_none();
    let new_config = config::Options {
        template: Some(String::from(DEFAULT_TEMPLATE_PATH)),
        ..options.config.to_owned()
    };
    let template_path = options
        .config
        .template
        .unwrap_or(DEFAULT_TEMPLATE_PATH.to_string());
    let src_read_result = fs::read(&options.config.src);

    if let Ok(content) = src_read_result {
        let read_result = fs::read(&template_path);

        if read_result.is_ok() {
            if !options.yes {
                let message = format!("Overwrite current template file {}", template_path);

                match ask_yes_no(&message, Answer::Yes) {
                    Answer::Yes => {} // just continue
                    Answer::No => {
                        return;
                    }
                }
            }
        }

        let content = String::from_utf8(content).unwrap();

        let result = parsing::parse(&content);

        if !result.valid {
            println!(
                "{} {} not valid.",
                "Error:".bold().red(),
                options.config.src
            );
            return;
        }

        let mut template_file = String::new();

        for key in result.keys {
            for constraint in key.constraints {
                let constraint_str = match constraint.value {
                    DecoratorValue::String(v) => format!("@{}(\"{}\")", constraint.key, v),
                    DecoratorValue::Integer(v) => format!("@{}({})", constraint.key, v),
                    DecoratorValue::None => format!("@{}", constraint.key),
                };
                template_file.push_str(&format!("{constraint_str}\n"));
            }

            template_file.push_str(&format!("{}\n", key.key));
        }

        fs::write(&template_path, template_file).unwrap();

        println!("Wrote new template file to {}", template_path);

        if no_configured_path {
            let new_config_content = serde_json::to_string_pretty(&new_config).unwrap();

            fs::write(CONFIG_PATH, new_config_content).unwrap();

            println!("Wrote template path to config file.");
        }
    } else {
        println!("Couldn't find a file at {}.", options.config.src);
    }
}
