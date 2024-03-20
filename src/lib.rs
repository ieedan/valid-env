use std::collections::HashMap;

use decorators::{DecoratorValidationResult, DecoratorValue};

pub mod decorators;

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

#[derive(Debug, Clone)]
pub struct Key {
    pub key: String,
    pub scope: Scope,
    pub value: ValueType,
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub keys: Vec<Key>,
}

impl ParseResult {
    pub fn valid(&self) -> bool {
        for key in self.keys.clone() {
            if !key.valid {
                return false;
            }
        }

        true
    }

    fn new() -> Self {
        ParseResult { keys: Vec::new() }
    }
}

pub fn parse(content: &str) -> ParseResult {
    let decorators: HashMap<String, decorators::Decorator> = decorators::get();

    let mut result = ParseResult::new();

    let chars: Vec<char> = content.trim().chars().collect();
    let len = chars.len();

    let mut is_value = false;
    let mut is_comment = false;
    let mut is_array = false;
    let mut is_decorator = false;
    let mut is_string = false;

    let mut current_key = String::new();
    let mut current = String::new();
    let mut current_decorators: Vec<String> = Vec::new();

    for (i, c) in chars.into_iter().enumerate() {
        if c == '@' && !is_value && !is_comment {
            is_decorator = true;
        } else if c == '=' && !is_decorator && !is_array && !is_value && !is_comment && !is_string {
            is_value = true;
            current_key = current.clone();
            current = String::new();
        } else if c == '"' && is_value && !i == len - 1 {
            is_string = !is_string;
            if is_array {
                current.push_str(&c.to_string());
            }
        } else if c == '[' && is_value && !is_array && !is_string {
            is_array = true;
        } else if c == ']' && is_value && is_array && !is_string {
            is_array = false;
        } else if (c == '\n' && !is_string && !is_array) || i == len - 1 {
            if i == len - 1 {
                current.push_str(&c.to_string());
            }

            if is_decorator {
                current_decorators.push(current.trim().to_owned());
                is_decorator = false;
            } else if is_comment {
                is_comment = false;
            } else if is_value {
                let mut errors: Vec<String> = Vec::new();

                let mut scope = Scope::Private;

                let value_type = coerce_value_type(&current.trim());

                for dec in current_decorators {
                    let start_parens = dec.find('(');
                    let decorator_info = match start_parens {
                        Some(index) => {
                            let end_parens = dec.find(')').unwrap_or(dec.len());

                            let key = &dec[0..index];
                            let value = &dec[index + 1..end_parens]; // This will get the value between the parentheses

                            (key, Some(value))
                        }
                        None => (&dec[..], None),
                    };

                    let found_decorator = decorators.get(decorator_info.0);

                    match found_decorator {
                        Some(d) => {
                            let dec_value = DecoratorValue::from_str(decorator_info.1);

                            let result = (d.validator)(value_type.to_owned(), dec_value);

                            if let DecoratorValidationResult::Error(err) = result {
                                errors.push(err);
                            }

                            if d.name == "public" {
                                scope = Scope::Public;
                            }
                        }
                        None => {
                            println!("WARN: invalid decorator '{}'", decorator_info.0);
                        }
                    }
                }

                let key = Key {
                    key: current_key.to_owned(),
                    scope: scope,
                    valid: errors.len() == 0,
                    value: value_type,
                    errors: errors,
                };

                result.keys.push(key);

                current_decorators = Vec::new();
                is_value = false;
            }

            // clear no matter what
            current = String::new();
        } else if !is_comment {
            // no need to add comments to current
            current.push_str(&c.to_string());
        }
    }

    result
}

fn coerce_value_type(val: &str) -> ValueType {
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

fn trim_quotes(val: &str) -> String {
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
