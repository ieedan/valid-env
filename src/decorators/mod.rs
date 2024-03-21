use crate::util::{trim_quotes, Compare};
use crate::{util::CompareResult, ValueType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DecoratorValue {
    String(String),
    Integer(f64),
    None,
}

impl DecoratorValue {
    pub fn from_str(val: &str) -> Self {
        if val.parse::<f64>().is_ok() {
            return DecoratorValue::Integer(val.parse::<f64>().unwrap());
        }

        return DecoratorValue::String(trim_quotes(val));
    }
}

pub struct DecoratorParseResult {
    pub key: String,
    pub value: DecoratorValue,
}

pub enum DecoratorValidationResult {
    Ok,
    Error(Vec<String>),
}

pub struct Decorator {
    pub name: String,
    pub validator: Box<dyn Fn(ValueType, DecoratorValue) -> DecoratorValidationResult>,
}

pub fn get() -> HashMap<String, Decorator> {
    let mut map: HashMap<String, Decorator> = HashMap::new();

    let decorators = vec![
        // ====== Private ======
        // This will modify the scope of the .env variable to private
        Decorator {
            name: String::from("private"),
            validator: Box::new(|_value, _decorator_value| DecoratorValidationResult::Ok),
        },
        // ====== Public ======
        // This will modify the scope of the .env variable to public
        Decorator {
            name: String::from("public"),
            validator: Box::new(|_value, _decorator_value| DecoratorValidationResult::Ok),
        },
        // ====== min ======
        // min compares the decorator value to the length of the string or the size of the number
        // If the value is a string or number array each value in the array is compared
        Decorator {
            name: String::from("min"),
            validator: Box::new(|value: ValueType, decorator_value: DecoratorValue| {
                match decorator_value {
                    DecoratorValue::String(v) => {
                        let error_message = format!("'{v}' is not valid for decorator type 'min'. 'min' requires a number value.");

                        return DecoratorValidationResult::Error(vec![error_message]);
                    }
                    DecoratorValue::Integer(dec_value) => match value {
                        ValueType::Number(v) => match v.cmp(&dec_value) {
                            CompareResult::Less => {
                                let error_message =
                                    format!("{v} is too small. Minimum value is {dec_value}.");

                                return DecoratorValidationResult::Error(vec![error_message]);
                            }
                            _ => return DecoratorValidationResult::Ok,
                        },
                        ValueType::String(v) => {
                            if dec_value > v.len() as f64 {
                                let error_message =
                                    format!("'{v}' is too short. Minimum length is {dec_value}.");

                                return DecoratorValidationResult::Error(vec![error_message]);
                            }

                            return DecoratorValidationResult::Ok;
                        }
                        ValueType::StringArray(v) => {
                            let mut errors: Vec<String> = Vec::new();
                            for s in v {
                                if dec_value > s.len() as f64 {
                                    let error_message = format!(
                                        "'{s}' is too short. Minimum length is {dec_value}."
                                    );

                                    errors.push(error_message);
                                }
                            }

                            if errors.len() != 0 {
                                return DecoratorValidationResult::Error(errors);
                            } else {
                                return DecoratorValidationResult::Ok;
                            }
                        }
                        ValueType::NumberArray(v) => {
                            let mut errors: Vec<String> = Vec::new();
                            for num in v {
                                match num.cmp(&dec_value) {
                                    CompareResult::Less => {
                                        let error_message = format!(
                                            "{num} is too small. Minimum value is {dec_value}."
                                        );

                                        errors.push(error_message);
                                    }
                                    _ => continue,
                                }
                            }

                            if errors.len() != 0 {
                                return DecoratorValidationResult::Error(errors);
                            } else {
                                return DecoratorValidationResult::Ok;
                            }
                        }
                    },
                    DecoratorValue::None => {
                        let error_message = format!("The min decorator requires a value to be provided with it. Ex: `@min(5)`");

                        return DecoratorValidationResult::Error(vec![error_message]);
                    }
                }
            }),
        },
        // ====== max ======
        // max compares the decorator value to the length of the string or the size of the number
        // If the value is a string or number array each value in the array is compared
        Decorator {
            name: String::from("max"),
            validator: Box::new(|value: ValueType, decorator_value: DecoratorValue| {
                match decorator_value {
                    DecoratorValue::String(v) => {
                        let error_message = format!("'{v}' is not valid for decorator type 'max'. 'max' requires a number value.");

                        return DecoratorValidationResult::Error(vec![error_message]);
                    }
                    DecoratorValue::Integer(dec_value) => match value {
                        ValueType::Number(v) => match v.cmp(&dec_value) {
                            CompareResult::Greater => {
                                let error_message =
                                    format!("{v} is too large. Maximum value is {dec_value}.");

                                return DecoratorValidationResult::Error(vec![error_message]);
                            }
                            _ => return DecoratorValidationResult::Ok,
                        },
                        ValueType::String(v) => {
                            if dec_value < v.len() as f64 {
                                let error_message =
                                    format!("'{v}' is too long. Maximum length is {dec_value}.");

                                return DecoratorValidationResult::Error(vec![error_message]);
                            }

                            return DecoratorValidationResult::Ok;
                        }
                        ValueType::StringArray(v) => {
                            let mut errors: Vec<String> = Vec::new();
                            for s in v {
                                if dec_value < s.len() as f64 {
                                    let error_message = format!(
                                        "'{s}' is too long. Maximum length is {dec_value}."
                                    );

                                    errors.push(error_message);
                                }
                            }

                            if errors.len() != 0 {
                                return DecoratorValidationResult::Error(errors);
                            } else {
                                return DecoratorValidationResult::Ok;
                            }
                        }
                        ValueType::NumberArray(v) => {
                            let mut errors: Vec<String> = Vec::new();
                            for num in v {
                                match num.cmp(&dec_value) {
                                    CompareResult::Greater => {
                                        let error_message = format!(
                                            "{num} is too large. Maximum value is {dec_value}."
                                        );

                                        println!("There was an error");

                                        errors.push(error_message);
                                    }
                                    _ => continue,
                                }
                            }

                            if errors.len() != 0 {
                                return DecoratorValidationResult::Error(errors);
                            } else {
                                return DecoratorValidationResult::Ok;
                            }
                        }
                    },
                    DecoratorValue::None => {
                        let error_message = format!("The max decorator requires a value to be provided with it. Ex: `@max(5)`");

                        return DecoratorValidationResult::Error(vec![error_message]);
                    }
                }
            }),
        },
        // ====== startsWith ======
        Decorator {
            name: String::from("startsWith"),
            validator: Box::new(|value, decorator_value| match decorator_value {
                DecoratorValue::Integer(dec_value) => {
                    let error_message = 
                        format!("'{dec_value}' is not valid for decorator type 'startsWith'. 'startsWith' requires a string value.");

                    return DecoratorValidationResult::Error(vec![error_message]);
                }
                DecoratorValue::String(dec_value) => {
                    match value {
                        ValueType::String(v) => {
                            if v.starts_with(&dec_value) {
                                return DecoratorValidationResult::Ok;
                            }

                            let error_message = format!("'{v}' does not start with '{dec_value}'");

                            return DecoratorValidationResult::Error(vec![error_message]);
                        },
                        ValueType::StringArray(v) => {
                            let mut errors: Vec<String> = Vec::new();
                            for item in v {
                                if !item.starts_with(&dec_value) {
                                    let error_message = format!("'{item}' does not start with '{dec_value}'");

                                    errors.push(error_message); 
                                }
                            }
                            
                            if errors.len() != 0 {
                                return DecoratorValidationResult::Error(errors);
                            } else {
                                return DecoratorValidationResult::Ok;
                            }
                        },
                        _ => {
                            let error_message = 
                            format!("startsWith does not support this variable type. startsWith only supports the string and string array types.");

                            return DecoratorValidationResult::Error(vec![error_message]);
                        }
                    }
                },
                DecoratorValue::None => {
                    let error_message = 
                        format!("The startsWith decorator requires a value to be provided with it. Ex: `@startsWith({})`", "\"index\"");

                    return DecoratorValidationResult::Error(vec![error_message]);
                }
            }),
        },
        // ====== endsWith ======
        Decorator {
            name: String::from("endsWith"),
            validator: Box::new(|value, decorator_value| match decorator_value {
                DecoratorValue::Integer(dec_value) => {
                    let error_message = 
                        format!("'{dec_value}' is not valid for decorator type 'endsWith'. 'endsWith' requires a string value.");

                    return DecoratorValidationResult::Error(vec![error_message]);
                }
                DecoratorValue::String(dec_value) => {
                    match value {
                        ValueType::String(v) => {
                            if v.ends_with(&dec_value) {
                                return DecoratorValidationResult::Ok;
                            }

                            let error_message = format!("'{v}' does not end with '{dec_value}'");

                            return DecoratorValidationResult::Error(vec![error_message]);
                        },
                        ValueType::StringArray(v) => {
                            let mut errors: Vec<String> = Vec::new();
                            for item in v {
                                if !item.ends_with(&dec_value) {
                                    let error_message = format!("'{item}' does not end with '{dec_value}'");

                                    errors.push(error_message); 
                                }
                            }
                            
                            if errors.len() != 0 {
                                return DecoratorValidationResult::Error(errors);
                            } else {
                                return DecoratorValidationResult::Ok;
                            }
                        },
                        _ => {
                            let error_message = 
                            format!("endsWith does not support this variable type. endsWith only supports the string and string array types.");

                            return DecoratorValidationResult::Error(vec![error_message]);
                        }
                    }
                },
                DecoratorValue::None => {
                    let error_message = 
                        format!("The endsWith decorator requires a value to be provided with it. Ex: `@endsWith({})`", "\"index\"");

                    return DecoratorValidationResult::Error(vec![error_message]);
                }
            }),
        },
    ];

    for dec in decorators {
        map.insert(dec.name.to_owned(), dec);
    }

    map
}

/// Parses the decorator syntax and returns its key and value if it has one
/// 
/// # Parameters
/// - `dec`: Should come in the format of `'decorator(value)'` or `'decorator'`
/// 
/// # Returns 
/// The key (before the parentheses) and value (inside of the parentheses) of the decorator
/// 
/// # Examples 
/// ```
/// let decorator = "startsWith("https");";
/// 
/// let decorator_info = parse(decorator);
/// 
/// assert_eq!(decorator_info.key, "startsWith");
/// ```
pub fn parse(dec: &str) -> DecoratorParseResult {
    let start_parens = dec.find('(');
    match start_parens {
        Some(index) => {
            let end_parens = dec.find(')').unwrap_or(dec.len());

            let key = &dec[0..index]; // Gets the value before the parentheses
            let value = &dec[index + 1..end_parens]; // Gets the value between the parentheses

            return DecoratorParseResult {
                key: key.to_owned(),
                value: DecoratorValue::from_str(value)
            }
        }
        None => {
            return DecoratorParseResult {
                key: dec.to_owned(),
                value: DecoratorValue::None
            }
        },
    };
}