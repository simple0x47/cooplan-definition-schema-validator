use crate::error::ErrorKind;

#[cfg(test)]
#[test]
fn validity_test_case_1() {
    use cooplan_definitions_lib::{
        definition::Definition, validated_source_attribute::ValidatedSourceAttribute,
        validated_source_category::ValidatedSourceCategory,
    };

    use crate::schema_validator::SchemaValidator;

    let json_value_string: String = String::from(
        "{ \"type\": \"2\", \"version\": \"1\", \"10\": \"Pear\", \"11\": 600, \"12\": 15.39, \"4ed908eb-50b6-4faa-9baa-a7a897cec30f\": true }",
    );

    let name_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "10".to_string(),
        name: "product_name".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };

    let count_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "11".to_string(),
        name: "count".to_string(),
        data_type: "integer".to_string(),
        unit: None,
        optional: false,
    };

    let price_per_kg_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "12".to_string(),
        name: "price_per_kg".to_string(),
        data_type: "decimal".to_string(),
        unit: None,
        optional: false,
    };

    let IS_PRODUCT: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "4ed908eb-50b6-4faa-9baa-a7a897cec30f".to_string(),
        name: "IS_PRODUCT".to_string(),
        data_type: "boolean".to_string(),
        unit: None,
        optional: false,
    };

    let product_category: ValidatedSourceCategory = ValidatedSourceCategory {
        id: "1".to_string(),
        parent: None,
        parent_name: None,
        name: "product".to_string(),
        selectable_as_last: false,
        attributes: vec![name_attribute, count_attribute, IS_PRODUCT],
    };

    let fruit_category: ValidatedSourceCategory = ValidatedSourceCategory {
        id: "2".to_string(),
        parent: Some("1".to_string()),
        parent_name: Some("product".to_string()),
        name: "fruit".to_string(),
        selectable_as_last: true,
        attributes: vec![price_per_kg_attribute],
    };

    let definition = Definition::new("1".to_string(), vec![product_category, fruit_category]);

    let mut schema_validator = SchemaValidator::default();

    assert!(schema_validator
        .validate(json_value_string, definition)
        .is_ok());
}

#[test]
fn validity_test_case_2() {
    use cooplan_definitions_lib::{
        definition::Definition, validated_source_attribute::ValidatedSourceAttribute,
        validated_source_category::ValidatedSourceCategory,
    };

    use crate::schema_validator::SchemaValidator;

    let json_value_string: String = String::from(
        "{ \"type\": \"2\", \"version\": \"1\", \"10\": 5, \"11\": 600, \"12\": 15.39, \"4ed908eb-50b6-4faa-9baa-a7a897cec30f\": true }",
    );

    let name_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "10".to_string(),
        name: "product_name".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };

    let count_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "11".to_string(),
        name: "count".to_string(),
        data_type: "integer".to_string(),
        unit: None,
        optional: false,
    };

    let price_per_kg_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "12".to_string(),
        name: "price_per_kg".to_string(),
        data_type: "decimal".to_string(),
        unit: None,
        optional: false,
    };

    let IS_PRODUCT: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "4ed908eb-50b6-4faa-9baa-a7a897cec30f".to_string(),
        name: "IS_PRODUCT".to_string(),
        data_type: "boolean".to_string(),
        unit: None,
        optional: false,
    };

    let product_category: ValidatedSourceCategory = ValidatedSourceCategory {
        id: "1".to_string(),
        parent: None,
        parent_name: None,
        name: "product".to_string(),
        selectable_as_last: false,
        attributes: vec![name_attribute, count_attribute, IS_PRODUCT],
    };

    let fruit_category: ValidatedSourceCategory = ValidatedSourceCategory {
        id: "2".to_string(),
        parent: Some("1".to_string()),
        parent_name: Some("product".to_string()),
        name: "fruit".to_string(),
        selectable_as_last: true,
        attributes: vec![price_per_kg_attribute],
    };

    let definition = Definition::new("1".to_string(), vec![product_category, fruit_category]);

    let mut schema_validator = SchemaValidator::default();

    assert_eq!(
        ErrorKind::InvalidValue,
        schema_validator
            .validate(json_value_string, definition)
            .unwrap_err()
            .kind()
    );
}

#[test]
fn validity_test_case_3() {
    use cooplan_definitions_lib::{
        definition::Definition, validated_source_attribute::ValidatedSourceAttribute,
        validated_source_category::ValidatedSourceCategory,
    };

    use crate::schema_validator::SchemaValidator;

    let json_value_string: String = String::from(
        "{ \"type\": \"2\", \"version\": \"1\", \"10\": \"Pear\", \"11\": 600, \"12\": 15.39, \"extra\": \"abcd\", \"4ed908eb-50b6-4faa-9baa-a7a897cec30f\": true }",
    );

    let name_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "10".to_string(),
        name: "product_name".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };

    let count_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "11".to_string(),
        name: "count".to_string(),
        data_type: "integer".to_string(),
        unit: None,
        optional: false,
    };

    let price_per_kg_attribute: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "12".to_string(),
        name: "price_per_kg".to_string(),
        data_type: "decimal".to_string(),
        unit: None,
        optional: false,
    };

    let IS_PRODUCT: ValidatedSourceAttribute = ValidatedSourceAttribute {
        id: "4ed908eb-50b6-4faa-9baa-a7a897cec30f".to_string(),
        name: "IS_PRODUCT".to_string(),
        data_type: "boolean".to_string(),
        unit: None,
        optional: false,
    };

    let product_category: ValidatedSourceCategory = ValidatedSourceCategory {
        id: "1".to_string(),
        parent: None,
        parent_name: None,
        name: "product".to_string(),
        selectable_as_last: false,
        attributes: vec![name_attribute, count_attribute, IS_PRODUCT],
    };

    let fruit_category: ValidatedSourceCategory = ValidatedSourceCategory {
        id: "2".to_string(),
        parent: Some("1".to_string()),
        parent_name: Some("product".to_string()),
        name: "fruit".to_string(),
        selectable_as_last: true,
        attributes: vec![price_per_kg_attribute],
    };

    let definition = Definition::new("1".to_string(), vec![product_category, fruit_category]);

    let mut schema_validator = SchemaValidator::default();

    let definition_value = schema_validator
        .validate(json_value_string, definition)
        .expect("failed to validate when there is an extra attribute");
    let definition_value_object = definition_value.value();

    assert!(definition_value_object.contains_key("11"));
    assert!(definition_value_object.contains_key("12"));

    assert!(!definition_value_object.contains_key("extra"));
}
