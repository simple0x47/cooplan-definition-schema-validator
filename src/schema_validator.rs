use cooplan_definitions_lib::definition::Definition;
use serde_json::{Map, Value};

use crate::error::{Error, ErrorKind};

const VALUE_VERSION: &str = "version";
const VALUE_TYPE: &str = "type";

pub struct SchemaValidator {}

impl SchemaValidator {
    pub fn validate(&self, value: String, definition: Definition) -> Result<(), Error> {
        match serde_json::from_str(value.as_str()) {
            Ok::<Map<String, Value>, _>(object) => {
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

                // TODO: GET TYPE'S ATTRIBTUES AND VALIDATE EACH VALUE ACCORDING TO ITS TYPE.

                Ok(())
            }
            Err(error) => Err(Error::new(
                ErrorKind::DeserializationFailure,
                format!("failed to deserialize value: {}", error),
            )),
        }
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
}
