use crate::parsing::{Scope, parse};

#[test]
fn correct_scope() {
    let expected_key = "SOMETHING";
    let expected_value = 9.0;

    let content = format!("@private\n{expected_key}={expected_value}");

    let result = parse(&content);

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

            match key.scope {
                Scope::Private => {},
                Scope::Public => panic!("Expected scope to be 'Private' got 'Public'")
            } 
        }
    }
}