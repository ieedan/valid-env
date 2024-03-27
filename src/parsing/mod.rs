use crate::decorators::{self, DecoratorValidationResult, ValidationError};
use crate::util::trim_quotes;
use std::collections::HashMap;

pub mod config;

#[derive(Debug, Clone, Copy)]
pub struct FilePosition {
    pub line: u32,
    pub column: u32,
}

impl FilePosition {
    pub fn new() -> Self {
        FilePosition { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone)]
pub enum Scope {
    Private,
    Public,
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Number(f64),
    String(String),
    StringArray(Vec<String>),
    NumberArray(Vec<f64>),
}

impl ValueType {
    pub fn to_string(&self) -> String {
        match self {
            ValueType::Number(v) => v.to_string(),
            ValueType::String(v) => format!("\"{v}\""),
            ValueType::StringArray(v) => format!("{:?}", v),
            ValueType::NumberArray(v) => format!("{:?}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    pub key: String,
    pub scope: Scope,
    pub value: ValueType,
    pub position: FilePosition,
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub decorators: Vec<decorators::DecoratorParseResult>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: FilePosition,
}

impl ParseError {
    pub fn new(msg: String, pos: FilePosition) -> Self {
        ParseError {
            message: msg,
            position: pos,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub keys: Vec<Key>,
    pub valid: bool,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseError>,
}

impl ParseResult {
    fn new() -> Self {
        ParseResult {
            keys: Vec::new(),
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

pub fn parse(content: &str) -> ParseResult {
    let decorators: HashMap<String, decorators::Decorator> = decorators::get();

    let mut result = ParseResult::new();

    let chars: Vec<char> = content.trim().chars().collect();
    let len = chars.len();

    let mut position = FilePosition::new();

    let mut is_value = false;
    let mut is_comment = false;
    let mut is_array = false;
    let mut is_decorator = false;
    let mut is_string = false;

    let mut current_key = (String::new(), FilePosition::new());
    let mut current = String::new();
    let mut current_decorators: Vec<(String, FilePosition)> = Vec::new();

    let mut keys: HashMap<String, Key> = HashMap::new();

    for (i, c) in chars.to_owned().into_iter().enumerate() {
        if c == '@' && !is_value && !is_comment && !is_decorator {
            is_decorator = true;
        } else if c == '=' && !is_decorator && !is_array && !is_value && !is_comment && !is_string {
            is_value = true;
            let mut key_position = position.to_owned();
            key_position.column = key_position
                .column
                .checked_sub(current.len() as u32)
                .unwrap();
            current_key = (current.clone(), key_position);
            current = String::new();
        } else if c == '"' && is_value {
            is_string = !is_string;
            current.push_str(&c.to_string());
        } else if c == '[' && is_value && !is_array && !is_string {
            is_array = true;
        } else if c == ']' && is_value && is_array && !is_string {
            is_array = false;
        } else if !is_comment {
            // no need to add comments to current
            current.push_str(&c.to_string());
        }

        if (c == '\n' && !is_string && !is_array) || i == len - 1 {
            if is_decorator {
                current_decorators.push((current.trim().to_owned(), position.to_owned()));
                is_decorator = false;
            } else if is_comment {
                is_comment = false;
            } else if is_value {
                let mut scope = Scope::Private;

                let value_type = coerce_value_type(&current.trim());

                let mut errors: Vec<ValidationError> = Vec::new();

                let mut constraints: Vec<decorators::DecoratorParseResult> = Vec::new();

                // Validate with decorators
                for (dec, pos) in current_decorators {
                    let decorator_info = decorators::parse(&dec);

                    let found_decorator = decorators.get(&decorator_info.key);

                    match found_decorator {
                        Some(d) => {
                            constraints.push(decorator_info.to_owned());

                            let result = (d.validator)(value_type.to_owned(), decorator_info.value);

                            if let DecoratorValidationResult::Error(errs) = result {
                                for err in errs {
                                    errors.push(err);
                                }
                            }

                            if d.name == "public" {
                                scope = Scope::Public;
                            }
                        }
                        None => {
                            let error_message =
                                format!("Invalid decorator '{}'", decorator_info.key);
                            result.errors.push(ParseError::new(error_message, pos));
                        }
                    }
                }

                let key = Key {
                    key: current_key.0.to_owned(),
                    valid: errors.len() == 0,
                    value: value_type,
                    position: current_key.1.to_owned(),
                    scope,
                    decorators: constraints,
                    errors,
                };

                if keys.contains_key(&key.key) {
                    let error_message = format!("Duplicate key '{}'", key.key);
                    result
                        .warnings
                        .push(ParseError::new(error_message, current_key.1.to_owned()));
                }

                keys.insert(key.key.to_owned(), key);

                current_key = (String::new(), FilePosition::new());
                current_decorators = Vec::new();
                is_value = false;
            }

            // clear no matter what
            current = String::new();
        }

        if c == '\n' {
            position.line += 1;
            position.column = 1;
        } else {
            position.column += 1;
        }
    }

    for (_, v) in keys {
        if !v.valid {
            result.valid = false;
        }

        result.keys.push(v);
    }

    if result.errors.len() > 0 {
        result.valid = false;
    }

    // Sorts the keys back to original order
    // since the hash map doesn't maintain the order
    result
        .keys
        .sort_by(|a, b| a.position.line.cmp(&b.position.line));

    result
}

/// Coerces the string value into a value type
pub fn coerce_value_type(val: &str) -> ValueType {
    // get value type
    if val.parse::<f64>().is_ok() {
        // number
        // Should be able to unwrap here because of is_ok
        return ValueType::Number(val.parse::<f64>().unwrap());
    } else {
        let chars: Vec<char> = val.chars().collect();
        let len = chars.len();

        let mut values: Vec<String> = Vec::new();

        let mut is_string = false;
        // Makes sure that if intended type is string it doesn't coerce to numbers
        let mut has_string = false;
        let mut current = String::new();

        for (i, c) in chars.into_iter().enumerate() {
            if c == '"' && !(i == len - 1) {
                has_string = true;
                is_string = !is_string;
            } else if (c == ',' && !is_string) || i == len - 1 {
                if i == len - 1 && !is_string {
                    current.push_str(&c.to_string());
                }
                values.push(current);
                current = String::new();
            } else if (c == ' ' && is_string) || c != ' ' {
                current.push_str(&c.to_string());
            }
        }

        if values.len() > 1 {
            if !has_string {
                // coerce to number array
                let mut are_numbers = true;

                let mut coerced_values: Vec<f64> = Vec::new();
                for v in values.clone() {
                    if v.parse::<f64>().is_ok() {
                        coerced_values.push(v.parse::<f64>().unwrap());
                    } else {
                        are_numbers = false;
                        break;
                    }
                }

                if !are_numbers {
                    // in the case that not all strings can be coerced to numbers return a string array
                    return ValueType::StringArray(values);
                }

                return ValueType::NumberArray(coerced_values);
            }

            return ValueType::StringArray(values);
        }

        return ValueType::String(trim_quotes(val));
    };
}
