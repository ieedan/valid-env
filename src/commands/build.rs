use std::fs;

use vnv::{decorators::DecoratorValue, parsing::{self, config}};

#[derive(Debug)]
pub struct Options {
    pub default: config::Options,
}

pub fn default(options: Options) {
    let file_content = fs::read(&options.default.src).unwrap();

    let content = String::from_utf8(file_content).unwrap();

    let result = parsing::parse(&content);

    let mut file = String::new();

    if !options.default.build.minify {
        file.push_str(&format!("# This file was generated from '{}' by vnv.\n\n", options.default.src))
    }

    for key in result.keys {
        if !options.default.build.minify {
            for constraint in key.constraints {
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

    fs::write(&options.default.build.output, file).unwrap();

    println!("Completed build wrote output to {}.", options.default.build.output)
}
