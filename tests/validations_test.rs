#[cfg(test)]
#[test]
fn detect_non_strings_correctly() {
    use serde_json::{Map, Number, Value};

    use cooplan_definition_schema_validator::validations::validate_string;

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
    use cooplan_definition_schema_validator::validations::validate_string;
    use serde_json::Value;

    let string_value: Value = Value::String(String::from("abcd"));
    let json_string_value: Value = Value::String(String::from("{ '1': 100 }"));

    assert!(validate_string(&string_value).is_ok());
    assert!(validate_string(&json_string_value).is_ok());
}

#[test]
fn detects_non_integer_correctly() {
    use serde_json::{Map, Number, Value};

    use cooplan_definition_schema_validator::validations::validate_integer;

    let null_value: Value = Value::Null;
    let numeric_array_value: Value = Value::Array(vec![
        Value::Number(Number::from_f64(12.13f64).unwrap()),
        Value::Number(Number::from_f64(43.12f64).unwrap()),
        Value::Number(Number::from_f64(7.51f64).unwrap()),
    ]);
    let decimal_value: Value = Value::Number(Number::from_f64(128.1f64).unwrap());
    let bool_value: Value = Value::Bool(false);
    let object_value: Value = Value::Object(Map::new());
    let string_value: Value = Value::String("12".to_string());

    assert!(validate_integer(&null_value).is_err());
    assert!(validate_integer(&numeric_array_value).is_err());
    assert!(validate_integer(&decimal_value).is_err());
    assert!(validate_integer(&bool_value).is_err());
    assert!(validate_integer(&object_value).is_err());
    assert!(validate_integer(&string_value).is_err());
}

#[test]
fn detects_integer_correctly() {
    use cooplan_definition_schema_validator::validations::validate_integer;
    use serde_json::{Number, Value};

    let positive_value: Value = Value::Number(Number::from(151531513));
    let negative_value: Value = Value::Number(Number::from(-1239));

    assert!(validate_integer(&positive_value).is_ok());
    assert!(validate_integer(&negative_value).is_ok());
}

#[test]
fn detects_non_decimal_correctly() {
    use serde_json::{Map, Number, Value};

    use cooplan_definition_schema_validator::validations::validate_decimal;

    let null_value: Value = Value::Null;
    let numeric_array_value: Value = Value::Array(vec![
        Value::Number(Number::from(2)),
        Value::Number(Number::from(25)),
        Value::Number(Number::from(6)),
    ]);
    let bool_value: Value = Value::Bool(false);
    let object_value: Value = Value::Object(Map::new());
    let string_value: Value = Value::String("134.13".to_string());

    assert!(validate_decimal(&null_value).is_err());
    assert!(validate_decimal(&numeric_array_value).is_err());
    assert!(validate_decimal(&bool_value).is_err());
    assert!(validate_decimal(&object_value).is_err());
    assert!(validate_decimal(&string_value).is_err());
}

#[test]
fn detects_decimal_correctly() {
    use cooplan_definition_schema_validator::validations::validate_decimal;
    use serde_json::{Number, Value};

    let positive_value: Value = Value::Number(Number::from_f64(151531513.135f64).unwrap());
    let negative_value: Value = Value::Number(Number::from_f64(-1239.13f64).unwrap());

    assert!(validate_decimal(&positive_value).is_ok());
    assert!(validate_decimal(&negative_value).is_ok());
}

#[test]
fn decimal_contains_number_set() {
    use cooplan_definition_schema_validator::validations::validate_decimal;
    use serde_json::{Number, Value};

    let positive_value: Value = Value::Number(Number::from(0138));
    let negative_value: Value = Value::Number(Number::from(-674));

    assert!(validate_decimal(&positive_value).is_ok());
    assert!(validate_decimal(&negative_value).is_ok());
}

#[test]
fn detects_non_boolean_correctly() {
    use serde_json::{Map, Number, Value};

    use cooplan_definition_schema_validator::validations::validate_boolean;

    let null_value: Value = Value::Null;
    let numeric_array_value: Value = Value::Array(vec![
        Value::Number(Number::from_f64(12.13f64).unwrap()),
        Value::Number(Number::from_f64(43.12f64).unwrap()),
        Value::Number(Number::from_f64(7.51f64).unwrap()),
    ]);
    let decimal_value: Value = Value::Number(Number::from_f64(513.1f64).unwrap());
    let integer_value: Value = Value::Number(Number::from(913));
    let object_value: Value = Value::Object(Map::new());
    let string_value: Value = Value::String("1".to_string());

    assert!(validate_boolean(&null_value).is_err());
    assert!(validate_boolean(&numeric_array_value).is_err());
    assert!(validate_boolean(&decimal_value).is_err());
    assert!(validate_boolean(&integer_value).is_err());
    assert!(validate_boolean(&object_value).is_err());
    assert!(validate_boolean(&string_value).is_err());
}

#[test]
fn detects_boolean_correctly() {
    use cooplan_definition_schema_validator::validations::validate_boolean;
    use serde_json::Value;

    let one: Value = Value::Bool(true);
    let zero: Value = Value::Bool(false);

    assert!(validate_boolean(&one).is_ok());
    assert!(validate_boolean(&zero).is_ok());
}
