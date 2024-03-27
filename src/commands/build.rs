use std::fs;

use vnv::{decorators::DecoratorValue, parsing::{self, config}};

use crate::commands::{self, check};

#[derive(Debug)]
pub struct Options {
    pub config: config::Options,
}

pub fn default(options: Options) {
    let file_content = fs::read(&options.config.src).unwrap();

    let content = String::from_utf8(file_content).unwrap();

    commands::check(check::Options {
        config: options.config.to_owned(),
        template: false
    });

    let result = parsing::parse(&content);

    let mut file = String::new();

    if !options.config.build.minify {
        file.push_str(&format!(
            "# This file was generated from '{}' by vnv.\n\n",
            options.config.src
        ))
    }

    for key in result.keys {
        if !options.config.build.minify {
            for constraint in key.decorators {
                let constraint_str = match constraint.value {
                    DecoratorValue::String(v) => format!("@{}(\"{}\")", constraint.key, v),
                    DecoratorValue::Integer(v) => format!("@{}({})", constraint.key, v),
                    DecoratorValue::None => format!("@{}", constraint.key),
                };
                file.push_str(&format!("# {constraint_str}\n"));
            }
        }

        file.push_str(&format!("{}={}\n", key.key, key.value.to_string()));
    }

    fs::write(&options.config.build.output, file).unwrap();

    println!(
        "Completed build wrote output to {}.",
        options.config.build.output
    )
}
