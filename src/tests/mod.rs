use crate::parsing::parse;

#[test]
fn string_correctly_parsed() {
    let content = r#"SOMETHING="something""#;

    let result = parse(content);

    for key in result.keys {
        if key.key != "SOMETHING" {
            panic!("'{}' was found. Expected 'SOMETHING'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::String(v) => {
                    if v != "something" {
                        panic!("'{v}' was found. Expected 'something'");
                    }
                }
                _ => panic!("Invalid value type expected String"),
            }
        }
    }
}

#[test]
fn number_correctly_parsed() {
    let num: f64 = 25.0;

    let content = format!("SOMETHING={num}");

    let result = parse(&content);

    for key in result.keys {
        if key.key != "SOMETHING" {
            panic!("'{}' was found. Expected 'SOMETHING'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::Number(v) => {
                    if v != num {
                        panic!("'{v}' was found. Expected '{num}'");
                    }
                }
                _ => panic!("Invalid value type expected Number"),
            }
        }
    }
}

#[test]
fn number_array_correctly_parsed() {
    let vec: Vec<f64> = vec![15.0, 20.0, 25.0, 30.0];

    let content = format!("SOMETHING={:?}", vec);

    let result = parse(&content);

    for key in result.keys {
        if key.key != "SOMETHING" {
            panic!("'{}' was found. Expected 'SOMETHING'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::NumberArray(v) => {
                    assert_eq!(vec, v);
                }
                _ => panic!("Invalid value type expected Number Array"),
            }
        }
    }
}

#[test]
fn string_array_correctly_parsed() {
    let vec: Vec<&str> = vec!["first", "second", "third", "fourth"];

    let content = format!("SOMETHING={:?}", vec);

    let result = parse(&content);

    for key in result.keys {
        if key.key != "SOMETHING" {
            panic!("'{}' was found. Expected 'SOMETHING'", key.key);
        } else {
            match key.value {
                crate::parsing::ValueType::StringArray(v) => {
                    assert_eq!(vec, v);
                }
                _ => panic!("Invalid value type expected String Array"),
            }
        }
    }
}
