use crate::parsing::parse;

#[test]
fn number_fails() {
    let max = 10;
    let expected_key = "SOMETHING";
    let expected_value = 11.0;

    let expected_error_message = format!("{expected_value} is too large. Maximum value is {max}.");

    let content = format!("@max({max})\n{expected_key}={expected_value}");

    let result = parse(&content);

    if result.valid {
        panic!("Result should have been invalid.");
    }

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::Number(v) => {
                    assert_eq!(expected_value, v);
                }
                _ => panic!("Invalid value type. Expected Number"),
            }

            for err in key.errors {
                assert_eq!(expected_error_message, err);
            }
        }
    }
}

#[test]
fn number_passes() {
    let max = 10;
    let expected_key = "SOMETHING";
    let expected_value = 10.0;

    let content = format!("@max({max})\n{expected_key}={expected_value}");

    let result = parse(&content);

    if !result.valid {
        panic!("Result should have been valid.");
    }

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::Number(v) => {
                    assert_eq!(expected_value, v);
                }
                _ => panic!("Invalid value type. Expected Number"),
            }
        }
    }
}

#[test]
fn string_fails() {
    let max = 5;
    let expected_key = "SOMETHING";
    let expected_value = "something";

    let expected_error_message =
        format!("'{expected_value}' is too long. Maximum length is {max}.");

    let content = format!("@max({max})\n{expected_key}={expected_value}");

    println!("{content}");

    let result = parse(&content);

    if result.valid {
        panic!("Result should have been invalid.");
    }

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::String(v) => {
                    assert_eq!(expected_value, v);
                }
                _ => panic!("Invalid value type. Expected String"),
            }

            for err in key.errors {
                assert_eq!(expected_error_message, err);
            }
        }
    }
}

#[test]
fn string_passes() {
    let max = 5;
    let expected_key = "SOMETHING";
    let expected_value = "some";

    let content = format!("@max({max})\n{expected_key}={expected_value}");

    let result = parse(&content);

    if !result.valid {
        panic!("Result should have been valid.");
    }

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::String(v) => {
                    assert_eq!(expected_value, v);
                }
                _ => panic!("Invalid value type. Expected String"),
            }
        }
    }
}
