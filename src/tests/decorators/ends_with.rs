use crate::parsing::parse;

#[test]
fn string_fails() {
    let ends_with = "@gmail.com";
    let expected_key = "SOMETHING";
    let expected_value = "aidan@yahoo.com";

    let expected_error_message =
        format!("'{expected_value}' does not end with '{ends_with}'");

    let content = format!("@endsWith(\"{ends_with}\")\n{expected_key}={expected_value}");

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
    let ends_with = "@gmail.com";
    let expected_key = "SOMETHING";
    let expected_value = "aidan@gmail.com";

    let content = format!("@endsWith(\"{ends_with}\")\n{expected_key}={expected_value}");

    println!("{content}");

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

#[test]
fn string_array_fails() {
    let ends_with = "@gmail.com";
    let expected_key = "SOMETHING";
    let failed_value = "someone@yahoo.com";
    let expected_value = vec![failed_value, "aidan@gmail.com", "john@gmail.com"];

    let expected_error_message =
        format!("'{failed_value}' does not end with '{ends_with}'");

    let content = format!("@endsWith(\"{ends_with}\")\n{expected_key}={:?}", expected_value);

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
                crate::parsing::ValueType::StringArray(v) => {
                    assert_eq!(expected_value, v);
                }
                _ => panic!("Invalid value type. Expected String Array"),
            }

            for err in key.errors {
                assert_eq!(expected_error_message, err);
            }
        }
    }
}

#[test]
fn string_array_passes() {
    let ends_with = "@gmail.com";
    let expected_key = "SOMETHING";
    let expected_value = vec!["someone@gmail.com", "aidan@gmail.com", "john@gmail.com"];

    let content = format!("@endsWith(\"{ends_with}\")\n{expected_key}={:?}", expected_value);

    println!("{content}");

    let result = parse(&content);

    if !result.valid {
        panic!("Result should have been valid.");
    }

    for key in result.keys {
        if key.key != expected_key {
            panic!("'{}' was found. Expected '{expected_key}'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::StringArray(v) => {
                    assert_eq!(expected_value, v);
                }
                _ => panic!("Invalid value type. Expected String Array"),
            }
        }
    }
}