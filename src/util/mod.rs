use std::{fmt::{format, Display}, io::{self, Write}, str::FromStr};
use colored::Colorize;

pub enum CompareResult {
    Less,
    Greater,
    Equal,
}

pub trait Compare {
    fn cmp(&self, other: &f64) -> CompareResult;
}

impl Compare for f64 {
    fn cmp(&self, other: &f64) -> CompareResult {
        if (*self - other).abs() < 1e-9 {
            CompareResult::Equal
        } else if *self < *other {
            CompareResult::Less
        } else {
            CompareResult::Greater
        }
    }
}

/// Trims quotes around the passed string
///  
/// # Examples
/// ```
/// let val = "\"something\"";
///
/// let result = vnv::util::trim_quotes(val);
///
/// assert_eq!(result, "something");
/// ```
pub fn trim_quotes(val: &str) -> String {
    let mut trimmed = String::from(val);

    let quote_start = val.find("\"");

    if let Some(start) = quote_start {
        let quote_end = val.rfind("\"");

        if let Some(end) = quote_end {
            trimmed = String::from(&val[start + 1..end]);
        }
    }

    trimmed
}

/// Adds whitespace to the left of the number so that it meets the min_length provided
///
/// # Returns
/// A left padding string containing the number
///
/// # Examples
///
/// ```
/// let number = 1;
///
/// let result = vnv::util::number_pad(number, 3);
///
/// assert_eq!(result, "  1");
/// ```
pub fn number_pad(num: u32, min_length: usize) -> String {
    let num_str = num.to_string();

    let padding = min_length - num_str.len();

    let mut result = String::new();

    for _ in 0..padding {
        result.push_str(" ");
    }

    result.push_str(&num_str);

    result
}

pub fn request_value<T>(value: &mut T, message: &str)
where
    T: FromStr + Display,
    <T as FromStr>::Err: std::fmt::Debug,
{
    printf(&format!(
        "{message} {}",
        value.to_string().truecolor(125, 125, 125)
    ));

    printf(&format!("\x1B[{}D", value.to_string().len()));

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    if !input.is_empty() {
        *value = input.parse::<T>().unwrap();
    }
}

pub fn printf(message: &str) {
    print!("{message}");
    io::stdout().flush().unwrap();
}

pub enum Answer {
    Yes,
    No,
}

pub fn read_yes_no() -> Answer {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            Answer::Yes
        }
        _ => {
            Answer::No
        }
    }
}
