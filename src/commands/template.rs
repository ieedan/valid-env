use std::fs;
use colored::Colorize;
use vnv::decorators::DecoratorValue;
use vnv::util::{ask_yes_no, Answer};

use vnv::parsing::{self, config};

pub struct Options {
    pub default: config::Options,
}

pub fn default(options: Options) {
    let src_read_result = fs::read(&options.default.src);

    if let Ok(content) = src_read_result {
        let read_result = fs::read(&options.default.template);

        if read_result.is_ok() {
            let message = format!("Overwrite current template file {}", options.default.template);

            match ask_yes_no(&message, Answer::No) {
                Answer::Yes => {} // just continue
                Answer::No => {
                    return;
                }
            }
        } else {
            return;
        }

        let content = String::from_utf8(content).unwrap();

        let result = parsing::parse(&content);

        if !result.valid {
            println!("{} {} not valid.", "Error:".bold().red(), options.default.src);
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

        fs::write(&options.default.template, template_file).unwrap();

        println!("Wrote new template file to {}", options.default.template);
    } else {
        println!("Couldn't find a file at {}.", options.default.src);
    }
}
