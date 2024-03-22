use std::{fs, path::Path, time::Instant};
use clap::Args;
use colored::Colorize;
use vnv::{parsing, util};

#[derive(Args, Debug)]
pub struct Options {
    /// Path of file to validate, defaults to ".vnv" if not specified.
    #[clap(short, long, value_parser, default_value = ".vnv")]
    file: String,

    /// Will hide the values in the ".vnv" from being output to the std out
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    cloak: bool,
}

pub fn default(options: Options) {
    let file = options.file;
    let cloak = options.cloak;
    
    let now = Instant::now();

    println!("Checking '{file}'...");

    let path = Path::new(&file.trim())
        .canonicalize()
        .expect("Must provide valid path.");

    let file_contents = fs::read(path).expect("Error reading file");

    let content = String::from_utf8(file_contents).unwrap();

    let result = parsing::parse(&content);

    let elapsed = now.elapsed();

    for key in result.keys {
        let mut status = String::new();
        if key.valid {
            status.push_str("✔️");
        } else {
            status.push_str("❌");
        }
        println!("{} {status}", key.key);
        for err in key.errors.clone() {
            let lines: Vec<&str> = content.split('\n').collect();
            let index = key.position.line - 1;
            let mut line = String::new();
            let mut error_squiggles = String::new();

            let string_val = match &key.value {
                vnv::parsing::ValueType::Number(v) => v.to_string(),
                vnv::parsing::ValueType::String(v) => format!("\"{v}\""),
                vnv::parsing::ValueType::StringArray(v) => format!("{:?}", v),
                vnv::parsing::ValueType::NumberArray(v) => format!("{:?}", v),
            };

            for (i, l) in lines.into_iter().enumerate() {
                if i == index as usize {
                    if let Some(val) = err.value {
                        let string_problem_val = match val {
                            vnv::parsing::ValueType::Number(v) => v.to_string(),
                            vnv::parsing::ValueType::String(v) => format!("\"{v}\""),
                            // These wont ever be hit but need to be implemented anyways
                            vnv::parsing::ValueType::StringArray(v) => format!("{:?}", v),
                            vnv::parsing::ValueType::NumberArray(v) => format!("{:?}", v),
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

            let mut error_str = String::new();

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
{ascii_arrow} {file}:{}:{}
     {ascii_line}
{}   {ascii_line}  {line}
     {ascii_line}  {}         
"#,
                error_str.bold(),
                key.position.line,
                key.position.column,
                util::number_pad(key.position.line, 2).to_string().blue(),
                error_squiggles.red()
            );
            println!("{error_message}");
        }
    }

    if !result.valid {
        println!("Check completed in {:.2?}", elapsed);
        panic!("Check failed!");
    } else {
        println!("Completed in {:.2?}", elapsed);
    }
}
