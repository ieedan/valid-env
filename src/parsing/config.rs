use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Options {
    pub src: String,
    pub cloak: bool,
    pub build: Build
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Build {
    pub output: String,
    pub minify: bool
}

impl Options {
    /// Returns the default values for config
    pub fn new() -> Self {
        Options {
            src: String::from(".vnv"),
            cloak: false,
            build: Build {
                output: String::from(".env"),
                minify: false
            }
        }
    }
}

pub fn parse(path: &str) -> Options {
    let defaults: Options = Options::new();

    let content = fs::read(path);

    if let Ok(content) = content {
        let json = String::from_utf8(content).unwrap();

        let object: Result<Value, _> = serde_json::from_str(&json);

        if let Ok(object) = object {
            // map the values to the options object
            return Options {
                src: object["src"].as_str().unwrap_or(&defaults.src).to_string(),
                cloak: object["cloak"].to_string().parse().unwrap_or(false),
                build: Build {
                    output: object["build"]["output"].as_str().unwrap_or(&defaults.build.output).to_string(),
                    minify: object["build"]["minify"].to_string().parse().unwrap_or(false)
                }
            };
        } else {
            return defaults;
        }
    } else {


        return defaults;
    }
}