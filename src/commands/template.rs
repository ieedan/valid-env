use colored::Colorize;
use std::fs;
use vnv::util::{printf, read_yes_no, Answer};

use vnv::parsing::{self, config};

pub struct Options {
    pub default: config::Options,
}

pub fn default(options: Options) {
    let src_read_result = fs::read(&options.default.src);

    if let Ok(content) = src_read_result {
        let read_result = fs::read(&options.default.template);

        if read_result.is_ok() {
            printf(&format!(
                "Overwrite current template file {} y/N? {}",
                options.default.template,
                "N".truecolor(125, 125, 125)
            ));

            printf("\x1B[1D");

            match read_yes_no() {
                Answer::Yes => {}
                Answer::No => {
                    return;
                }
            }
        } else {
            return;
        }

        let content = String::from_utf8(content).unwrap();

        let result = parsing::parse(&content);
    } else {
        println!("Couldn't find a file at {}.", options.default.src);
    }
}
