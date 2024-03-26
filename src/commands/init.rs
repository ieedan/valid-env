use colored::Colorize;
use std::{
    fmt::{format, Display},
    fs,
    io::{self, Write},
    str::FromStr,
};
use vnv::parsing::config;

const DEFAULT_SRC: &str = r#"@matches("bar")
FOO="bar"#;

const DEFAULT_TEMPLATE: &str = r#"# Use this file to scaffold what the '.vnv' file should look like without values
# This file should be committed to your source control and vnv will verify that the '.vnv' file matches this files template 

@matches("bar")
FOO"#;

/// Initializes the config file and optionally a template file
pub fn default() {
    println!(r#"                  
    __   ___ ____   __
    \ \ / / '_ \ \ / /
     \ V /| | | \ V / 
      \_/ |_| |_|\_/  
                      
"#);

    let mut config = config::Options::new();

    request_value(&mut config.src, "Where is the source file?");

    let result = fs::read(&config.src);

    if result.is_ok() {
        print(&format!(
            "Overwrite source file y/N? '{}' : ",
            "N".dimmed().italic()
        ));

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                println!("Overwriting source file at {}", config.src);

                fs::write(config.src, DEFAULT_SRC).unwrap();
            }
            _ => {}
        }
    } else {
        println!("Creating source file at {}", config.src);

        fs::write(&config.src, DEFAULT_SRC).unwrap();
    }

    let mut input = String::new();

    print(&format!(
        "Use a template file y/N? '{}'",
        "N".dimmed().italic()
    ));

    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            request_value(&mut config.template, "Where is the template file?");

            let result = fs::read(&config.template);

            if result.is_ok() {
                print(&format!(
                    "Overwrite template file y/N? '{}' : ",
                    "N".dimmed().italic()
                ));

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => {
                        println!("Overwriting template file at {}", config.template);

                        fs::write(config.template, DEFAULT_TEMPLATE).unwrap();
                    }
                    _ => {}
                }
            } else {
                println!("Creating template file at {}", config.template);

                fs::write(config.template, DEFAULT_TEMPLATE).unwrap();
            }
        }
        _ => {}
    }
}

fn request_value<T>(value: &mut T, message: &str)
where
    T: FromStr + Display,
    <T as FromStr>::Err: std::fmt::Debug,
{
    print(&format!(
        "{message} '{}': ",
        value.to_string().dimmed().italic()
    ));

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    if !input.is_empty() {
        *value = input.parse::<T>().unwrap();
    }
}

fn print(message: &str) {
    print!("{message}");
    io::stdout().flush().unwrap();
}
