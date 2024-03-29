use std::collections::HashMap;

use cooplan_definitions_lib::{
    definition::Definition, validated_source_attribute::ValidatedSourceAttribute,
    validated_source_category::ValidatedSourceCategory,
};
use serde_json::{Map, Value};

use crate::category_chain::{build_from_definition_and_category, CategoryChain};
use crate::{
    definition_value::DefinitionValue,
    error::{Error, ErrorKind},
    validations::{validate_boolean, validate_decimal, validate_integer, validate_string},
};

const VALUE_VERSION: &str = "version";
const VALUE_TYPE: &str = "type";

type Validation = Box<dyn Fn(&Value) -> Result<(), Error> + Send>;

pub struct SchemaValidator {
    categories: HashMap<String, ValidatedSourceCategory>,
    validations: HashMap<String, Validation>,
}

impl SchemaValidator {
    fn initialize_base_validations(validations: &mut HashMap<String, Validation>) {
        validations.insert("string".to_string(), Box::new(validate_string));
        validations.insert("integer".to_string(), Box::new(validate_integer));
        validations.insert("decimal".to_string(), Box::new(validate_decimal));
        validations.insert("boolean".to_string(), Box::new(validate_boolean));
    }

    pub fn register_validation(&mut self, data_type: String, validation: Validation) {
        self.validations.insert(data_type, validation);
    }

    pub fn validate(
        &mut self,
        value: String,
        definition: Definition,
    ) -> Result<DefinitionValue, Error> {
        match serde_json::from_str(value.as_str()) {
            Ok::<Map<String, Value>, _>(mut object) => {
                let value_definition_version = match self.try_get_version(&object) {
                    Ok(version) => version,
                    Err(error) => return Err(error),
                };

                if definition.version().to_lowercase() != value_definition_version.to_lowercase() {
                    return Err(Error::new(
                        ErrorKind::ValueDefinitionMismatch,
                        format!(
                            "value's version '{}' != definition's version '{}'",
                            value_definition_version,
                            definition.version()
                        ),
                    ));
                }

                let value_type = match self.try_get_type(&object) {
                    Ok(value_type) => value_type,
                    Err(error) => return Err(error),
                };

                let attributes: Vec<ValidatedSourceAttribute> =
                    match self.try_get_attributes_from_definition(&definition, &value_type) {
                        Ok(attributes) => attributes,
                        Err(error) => return Err(error),
                    };

                let mut scoped_value: Map<String, Value> = Map::new();

                for attribute in attributes.as_slice() {
                    let attribute_value = match object.remove(&attribute.id) {
                        Some(attribute_value) => attribute_value,
                        None => {
                            return Err(Error::new(
                                ErrorKind::InvalidValue,
                                format!(
                                    "failed to find attribute id '{}' within value",
                                    attribute.id
                                ),
                            ))
                        }
                    };

                    match self.validations.get(&attribute.data_type) {
                        Some(validation) => match validation(&attribute_value) {
                            Ok(_) => {
                                scoped_value.insert(attribute.id.clone(), attribute_value);
                            }
                            Err(error) => return Err(error),
                        },
                        None => {
                            return Err(Error::new(
                                ErrorKind::ValidationNotRegistered,
                                format!(
                                    "no validation found for data type '{}'",
                                    attribute.data_type
                                ),
                            ))
                        }
                    }
                }

                let category_chain = build_from_definition_and_category(&definition, &value_type);

                let definition_value =
                    DefinitionValue::try_new(&definition, category_chain, scoped_value)?;

                Ok(definition_value)
            }
            Err(error) => Err(Error::new(
                ErrorKind::DeserializationFailure,
                format!("failed to deserialize value: {}", error),
            )),
        }
    }

    pub fn validate_object(
        &mut self,
        mut object: Map<String, Value>,
        definition: Definition,
    ) -> Result<DefinitionValue, Error> {
        let value_definition_version = match self.try_get_version(&object) {
            Ok(version) => version,
            Err(error) => return Err(error),
        };

        if definition.version().to_lowercase() != value_definition_version.to_lowercase() {
            return Err(Error::new(
                ErrorKind::ValueDefinitionMismatch,
                format!(
                    "value's version '{}' != definition's version '{}'",
                    value_definition_version,
                    definition.version()
                ),
            ));
        }

        let value_type = match self.try_get_type(&object) {
            Ok(value_type) => value_type,
            Err(error) => return Err(error),
        };

        let attributes: Vec<ValidatedSourceAttribute> =
            match self.try_get_attributes_from_definition(&definition, &value_type) {
                Ok(attributes) => attributes,
                Err(error) => return Err(error),
            };

        let mut scoped_value: Map<String, Value> = Map::new();

        for attribute in attributes.as_slice() {
            let attribute_value = match object.remove(&attribute.id) {
                Some(attribute_value) => attribute_value,
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidValue,
                        format!(
                            "failed to find attribute id '{}' within value",
                            attribute.id
                        ),
                    ))
                }
            };

            match self.validations.get(&attribute.data_type) {
                Some(validation) => match validation(&attribute_value) {
                    Ok(_) => {
                        scoped_value.insert(attribute.id.clone(), attribute_value);
                    }
                    Err(error) => return Err(error),
                },
                None => {
                    return Err(Error::new(
                        ErrorKind::ValidationNotRegistered,
                        format!(
                            "no validation found for data type '{}'",
                            attribute.data_type
                        ),
                    ))
                }
            }
        }

        let category_chain = build_from_definition_and_category(&definition, &value_type);

        let definition_value = DefinitionValue::try_new(&definition, category_chain, scoped_value)?;

        Ok(definition_value)
    }

    fn try_get_version(&self, object: &Map<String, Value>) -> Result<String, Error> {
        match object.get(VALUE_VERSION) {
            Some(version) => match version.as_str() {
                Some(version_string) => Ok(version_string.to_string()),
                None => Err(Error::new(
                    ErrorKind::InvalidValue,
                    "version attribute's value cannot be read as a string".to_string(),
                )),
            },
            None => Err(Error::new(
                ErrorKind::VersionAttributeMissing,
                "version attribute is missing".to_string(),
            )),
        }
    }

    fn try_get_type(&self, object: &Map<String, Value>) -> Result<String, Error> {
        match object.get(VALUE_TYPE) {
            Some(value_type) => match value_type.as_str() {
                Some(type_string) => Ok(type_string.to_string()),
                None => Err(Error::new(
                    ErrorKind::InvalidValue,
                    "type attribute's value cannot be read as a string".to_string(),
                )),
            },
            None => Err(Error::new(
                ErrorKind::TypeAttributeMissing,
                "type attribute is missing".to_string(),
            )),
        }
    }

    fn try_get_attributes_from_definition(
        &mut self,
        definition: &Definition,
        category_id: &String,
    ) -> Result<Vec<ValidatedSourceAttribute>, Error> {
        self.categories.clear();

        for category in definition.categories() {
            self.categories.insert(category.id.clone(), category);
        }

        let mut attributes: Vec<ValidatedSourceAttribute> = Vec::new();

        match self.add_attributes_from_category(&mut attributes, category_id) {
            Ok(_) => Ok(attributes),
            Err(error) => Err(error),
        }
    }

    fn add_attributes_from_category(
        &self,
        attributes: &mut Vec<ValidatedSourceAttribute>,
        category_id: &String,
    ) -> Result<(), Error> {
        let category = match self.categories.get(category_id) {
            Some(category) => category,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidValue,
                    format!("cannot found category id '{}'", category_id),
                ))
            }
        };

        for attribute in category.attributes.as_slice() {
            attributes.push(attribute.clone());
        }

        match &category.parent {
            Some(parent_category_id) => {
                self.add_attributes_from_category(attributes, &parent_category_id)
            }
            None => Ok(()),
        }
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        let mut validations: HashMap<String, Validation> = HashMap::new();

        SchemaValidator::initialize_base_validations(&mut validations);

        SchemaValidator {
            categories: HashMap::new(),
            validations,
        }
    }
}
