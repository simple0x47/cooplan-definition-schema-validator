#[cfg(test)]
#[test]
fn detect_non_strings_correctly() {
    use serde_json::{Map, Number, Value};

    use crate::validations::validate_string;

    let null_value: Value = Value::Null;
    let numeric_array_value: Value = Value::Array(vec![
        Value::Number(Number::from(2)),
        Value::Number(Number::from(25)),
        Value::Number(Number::from(6)),
    ]);
    let number_value: Value = Value::Number(Number::from(10));
    let bool_value: Value = Value::Bool(false);
    let object_value: Value = Value::Object(Map::new());

    assert!(validate_string(&null_value).is_err());
    assert!(validate_string(&numeric_array_value).is_err());
    assert!(validate_string(&number_value).is_err());
    assert!(validate_string(&bool_value).is_err());
    assert!(validate_string(&object_value).is_err());
}

#[test]
fn detect_strings_correctly() {
    use crate::validations::validate_string;
    use serde_json::Value;

    let string_value: Value = Value::String(String::from("abcd"));
    let json_string_value: Value = Value::String(String::from("{ '1': 100 }"));

    assert!(validate_string(&string_value).is_ok());
    assert!(validate_string(&json_string_value).is_ok());
}

#[test]
fn detects_non_integer_correctly() {
    use serde_json::{Map, Number, Value};

    use crate::validations::validate_integer;

    let null_value: Value = Value::Null;
    let numeric_array_value: Value = Value::Array(vec![
        Value::Number(Number::from_f64(12.13f64).unwrap()),
        Value::Number(Number::from_f64(43.12f64).unwrap()),
        Value::Number(Number::from_f64(7.51f64).unwrap()),
    ]);
    let decimal_value: Value = Value::Number(Number::from_f64(128.1f64).unwrap());
    let bool_value: Value = Value::Bool(false);
    let object_value: Value = Value::Object(Map::new());

    assert!(validate_integer(&null_value).is_err());
    assert!(validate_integer(&numeric_array_value).is_err());
    assert!(validate_integer(&decimal_value).is_err());
    assert!(validate_integer(&bool_value).is_err());
    assert!(validate_integer(&object_value).is_err());
}

#[test]
fn detects_integer_correctly() {
    use crate::validations::validate_integer;
    use serde_json::{Number, Value};

    let positive_value: Value = Value::Number(Number::from(151531513));
    let negative_value: Value = Value::Number(Number::from(-1239));

    assert!(validate_integer(&positive_value).is_ok());
    assert!(validate_integer(&negative_value).is_ok());
}
