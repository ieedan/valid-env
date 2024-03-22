use crate::parsing::parse;

#[test]
fn string_fails() {
    let starts_with = "https://";
    let expected_key = "SOMETHING";
    let expected_value = "http://google.com";

    let expected_error_message =
        format!("'{expected_value}' does not start with '{starts_with}'");

    let content = format!("@startsWith(\"{starts_with}\")\n{expected_key}={expected_value}");

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
    let starts_with = "https://";
    let expected_key = "SOMETHING";
    let expected_value = "https://google.com";

    let content = format!("@startsWith(\"{starts_with}\")\n{expected_key}={expected_value}");

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
    let starts_with = "https://";
    let expected_key = "SOMETHING";
    let failed_value = "http://google.com";
    let expected_value = vec![failed_value, "https://github.com", "https://doc.rust-lang.org"];

    let expected_error_message =
        format!("'{failed_value}' does not start with '{starts_with}'");

    let content = format!("@startsWith(\"{starts_with}\")\n{expected_key}={:?}", expected_value);

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
    let starts_with = "https://";
    let expected_key = "SOMETHING";
    let expected_value = vec!["https://google.com", "https://github.com", "https://doc.rust-lang.org"];

    let content = format!("@startsWith(\"{starts_with}\")\n{expected_key}={:?}", expected_value);

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