use crate::parsing::parse;

#[test]
fn string_correctly_parsed() {
    let expected_key = "SOMETHING";
    let expected_val = "something";

    let content = format!("{expected_key}=\"{expected_val}\"");

    let result = parse(&content);

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::String(v) => {
                    if v != expected_val {
                        panic!("'{v}' was found. Expected '{expected_val}'");
                    }
                }
                _ => panic!("Invalid value type. Expected String"),
            }
        }
    }
}

#[test]
fn number_correctly_parsed() {
    let expected_key = "SOMETHING";
    let expected_val: f64 = 25.0;

    let content = format!("{expected_key}={expected_val}");

    let result = parse(&content);

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::Number(v) => {
                    if v != expected_val {
                        panic!("'{v}' was found. Expected '{expected_val}'");
                    }
                }
                _ => panic!("Invalid value type. Expected Number"),
            }
        }
    }
}

#[test]
fn number_array_correctly_parsed() {
    let expected_key = "SOMETHING";
    let expected_val: Vec<f64> = vec![15.0, 20.0, 25.0, 30.0];

    let content = format!("{expected_key}={:?}", expected_val);

    let result = parse(&content);

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::NumberArray(v) => {
                    assert_eq!(expected_val, v);
                }
                _ => panic!("Invalid value type. Expected Number Array"),
            }
        }
    }
}

#[test]
fn string_array_correctly_parsed() {
    let expected_key = "SOMETHING";
    let expected_val: Vec<&str> = vec!["first", "second", "third", "fourth"];

    let content = format!("{expected_key}={:?}", expected_val);

    let result = parse(&content);

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::StringArray(v) => {
                    assert_eq!(expected_val, v);
                }
                _ => panic!("Invalid value type. Expected String Array"),
            }
        }
    }
}
