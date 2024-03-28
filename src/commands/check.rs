use colored::Colorize;
use core::panic;
use std::{fs, time::Instant};
use vnv::{
    parsing::{self, config, Environment, ValueType},
    util,
};

#[derive(Debug)]
pub struct Options {
    pub config: config::Options,
    pub environment: Environment,
}

// src file does not match template file. If this is intended you can run `vnv template` to re-create the template file based on the src file.

pub fn default(options: Options) {
    let cloak = options.config.cloak;

    let now = Instant::now();

    println!("Checking '{}'...", options.config.src);

    let src_contents = fs::read(&options.config.src).expect("Error reading file");

    let content = String::from_utf8(src_contents).unwrap();

    let result = parsing::parse(&content);

    let mut valid = true; 

    for key in result.keys {
        let environment_does_not_match = key.environment != options.environment;
        let key_environment_not_all = key.environment != Environment::All;

        if environment_does_not_match && key_environment_not_all {
            println!("{} ⏭️", key.key.truecolor(125, 125, 125));
            continue; // skip this key
        }

        let mut status = String::new();
        if key.valid {
            status.push_str("✔️");
        } else {
            valid = false;
            status.push_str("❌");
        }
        println!("{} {status}", key.key);
        for err in key.errors.clone() {
            let lines: Vec<&str> = content.split('\n').collect();
            let index = key.position.line - 1;
            let mut line = String::new();
            let mut error_squiggles = String::new();

            let string_val = match &key.value {
                ValueType::Number(v) => v.to_string(),
                ValueType::String(v) => format!("\"{v}\""),
                ValueType::StringArray(v) => format!("{:?}", v),
                ValueType::NumberArray(v) => format!("{:?}", v),
            };

            for (i, l) in lines.into_iter().enumerate() {
                if i == index as usize {
                    if let Some(val) = err.value {
                        let string_problem_val = match val {
                            ValueType::Number(v) => v.to_string(),
                            ValueType::String(v) => format!("\"{v}\""),
                            // These wont ever be hit but need to be implemented anyways
                            ValueType::StringArray(v) => format!("{:?}", v),
                            ValueType::NumberArray(v) => format!("{:?}", v),
                        };

                        let start_index = string_val.find(&string_problem_val);

                        if let Some(index) = start_index {
                            // Calculates the start from the '=' and then adds the index
                            let offset: u32 = key.key.len() as u32 + 1 + index as u32;

                            // Left pad
                            for _ in 0..offset {
                                error_squiggles.push_str(" ");
                            }

                            for _ in 0..string_problem_val.len() {
                                error_squiggles.push_str("^");
                            }
                        }

                        if cloak {
                            let mut cloaked = String::new();

                            for _ in 0..string_val.len() {
                                cloaked.push_str("*");
                            }

                            line = format!("{}={}", key.key, cloaked);
                        } else {
                            line = format!("{}={string_val}", key.key);
                        }
                    } else {
                        if cloak {
                            let found_index = l.find("=");

                            if let Some(index) = found_index {
                                let start = &l[..index + 1];

                                let mut cloaked = String::new();

                                for _ in 0..string_val.len() {
                                    cloaked.push_str("*");
                                }

                                line = format!("{start}{cloaked}");
                            }
                        } else {
                            line = l.to_string();
                        }
                    }
                    break;
                }
            }

            let error_str: String;

            if cloak {
                let trimmed_val = util::trim_quotes(&string_val);
                let found_index = err.message.find(&trimmed_val);

                if let Some(index) = found_index {
                    println!("{} found at index {}", err.message, index);

                    let start = &err.message[..index];
                    let mut cloaked = String::new();

                    for _ in 0..string_val.len() {
                        cloaked.push_str("*");
                    }
                    let end = &err.message[index + trimmed_val.len()..];

                    error_str = format!("{start}{cloaked}{end}");
                } else {
                    error_str = err.message;
                }
            } else {
                error_str = format!("{}", err.message);
            }

            let ascii_error = "ERROR".red().bold();
            let ascii_line = "|".blue();
            let ascii_arrow = "-->".blue();
            let error_message = format!(
                r#"{ascii_error}: {}
{ascii_arrow} {}:{}:{}
     {ascii_line}
{}   {ascii_line}  {line}
     {ascii_line}  {}         
"#,
                error_str.bold(),
                options.config.src,
                key.position.line,
                key.position.column,
                util::number_pad(key.position.line, 2).to_string().blue(),
                error_squiggles.red()
            );
            println!("{error_message}");
        }
    }

    let elapsed = now.elapsed();

    if !valid {
        println!("Check completed in {:.2?}", elapsed);
        panic!("Check failed!");
    } else {
        println!("Completed in {:.2?}", elapsed);
    }
}
