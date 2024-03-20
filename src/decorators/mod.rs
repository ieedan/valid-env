use crate::ValueType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DecoratorValue {
    String(String),
    Integer(f64),
    None,
}

impl DecoratorValue {
    pub fn from_str(val: Option<&str>) -> Self {
        match val {
            Some(v) => {
                if v.parse::<f64>().is_ok() {
                    return DecoratorValue::Integer(v.parse::<f64>().unwrap());
                }

                return DecoratorValue::String(v.to_owned());
            },
            None => DecoratorValue::None,
        }
    }
}

pub enum DecoratorValidationResult {
    Ok,
    Error(String),
}

pub struct Decorator {
    pub name: String,
    pub validator: Box<dyn Fn(ValueType, DecoratorValue) -> DecoratorValidationResult>,
}

pub enum CompareResult {
    Less,
    Greater,
    Equal,
}

trait Compare {
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

                        return DecoratorValidationResult::Error(error_message);
                    }
                    DecoratorValue::Integer(dec_value) => match value {
                        ValueType::Number(v) => match v.cmp(&dec_value) {
                            CompareResult::Less => {
                                let error_message = format!("{v} is too small. Minimum value is {dec_value}.");

                                return DecoratorValidationResult::Error(error_message);
                            }
                            _ => return DecoratorValidationResult::Ok,
                        },
                        ValueType::String(v) => {
                            if dec_value > v.len() as f64 {
                                let error_message = format!("'{v}' is too short. Minimum length is {dec_value}.");

                                return DecoratorValidationResult::Error(error_message);
                            }

                            return DecoratorValidationResult::Ok;
                        }
                        ValueType::StringArray(v) => {
                            for s in v {
                                if dec_value > s.len() as f64 {
                                    let error_message = format!("'{s}' is too short. Minimum length is {dec_value}.");

                                    return DecoratorValidationResult::Error(error_message);
                                }
                            }

                            return DecoratorValidationResult::Ok;
                        }
                        ValueType::NumberArray(v) => {
                            for num in v {
                                match num.cmp(&dec_value) {
                                    CompareResult::Less => {
                                        let error_message = format!("{num} is too small. Minimum value is {dec_value}.");

                                        return DecoratorValidationResult::Error(error_message);
                                    }
                                    _ => continue,
                                }
                            }

                            return DecoratorValidationResult::Ok;
                        }
                    },
                    DecoratorValue::None => {
                        let error_message = format!("The min decorator requires a value to be provided with it. Ex: `@min(5)`");

                        return DecoratorValidationResult::Error(error_message);
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

                        return DecoratorValidationResult::Error(error_message);
                    }
                    DecoratorValue::Integer(dec_value) => match value {
                        ValueType::Number(v) => match v.cmp(&dec_value) {
                            CompareResult::Greater => {
                                let error_message = format!("{v} is too large. Maximum value is {dec_value}.");

                                return DecoratorValidationResult::Error(error_message);
                            }
                            _ => return DecoratorValidationResult::Ok,
                        },
                        ValueType::String(v) => {
                            if dec_value < v.len() as f64 {
                                let error_message = format!("'{v}' is too long. Maximum length is {dec_value}.");

                                return DecoratorValidationResult::Error(error_message);
                            }

                            return DecoratorValidationResult::Ok;
                        }
                        ValueType::StringArray(v) => {
                            for s in v {
                                if dec_value < s.len() as f64 {
                                    let error_message = format!("'{s}' is too long. Maximum length is {dec_value}.");

                                    return DecoratorValidationResult::Error(error_message);
                                }
                            }

                            return DecoratorValidationResult::Ok;
                        }
                        ValueType::NumberArray(v) => {
                            for num in v {
                                match num.cmp(&dec_value) {
                                    CompareResult::Greater => {
                                        let error_message = format!("{num} is too large. Maximum value is {dec_value}.");

                                        println!("There was an error");

                                        return DecoratorValidationResult::Error(error_message);
                                    }
                                    _ => continue,
                                }
                            }

                            DecoratorValidationResult::Ok
                        }
                    },
                    DecoratorValue::None => {
                        let error_message = format!("The max decorator requires a value to be provided with it. Ex: `@max(5)`");

                        return DecoratorValidationResult::Error(error_message);
                    }
                }
            }),
        },
    ];

    for dec in decorators {
        map.insert(dec.name.to_owned(), dec);
    }

    map
}
