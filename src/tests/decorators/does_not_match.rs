use crate::parsing::parse;

#[test]
fn string_fails() {
    // Matches digits only
    let pattern = "^\\d+$";
    let expected_key = "SOMETHING";
    let expected_value = "123456";

    let expected_error_message =
        format!("'{expected_value}' matches '{pattern}'.");

    let content = format!("@doesNotMatch(\"{pattern}\")\n{expected_key}=\"{expected_value}\"");

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
                assert_eq!(expected_error_message, err.message);
            }
        }
    }
}

#[test]
fn string_passes() {
    // Matches digits only
    let pattern = "^\\d+$";
    let expected_key = "SOMETHING";
    let expected_value = "1234five6";

    let content = format!("@doesNotMatch(\"{pattern}\")\n{expected_key}=\"{expected_value}\"");

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
    // Matches digits only
    let pattern = "^\\d+$";
    let expected_key = "SOMETHING";
    let failed_value = "123456";
    let expected_value = vec![failed_value, "six54321", "one23456"];

    let expected_error_message =
        format!("'{failed_value}' matches '{pattern}'.");

    let content = format!("@doesNotMatch(\"{pattern}\")\n{expected_key}={:?}", expected_value);

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
                assert_eq!(expected_error_message, err.message);
            }
        }
    }
}

#[test]
fn string_array_passes() {
    // Matches digits only
    let pattern = "^\\d+$";
    let expected_key = "SOMETHING";
    let expected_value = vec!["1234five6", "six54321", "one23456"];

    let content = format!("@doesNotMatch(\"{pattern}\")\n{expected_key}={:?}", expected_value);

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