use cooplan_definitions_lib::definition::Definition;
use serde_json::{Map, Value};
use crate::definition_type::DefinitionType;

/// Validated, definition scoped, JSON value.
#[derive(Debug)]
pub struct DefinitionValue {
    definition: String,
    value: Map<String, Value>,
}

impl DefinitionValue {
    pub fn new(definition: &Definition, value: Map<String, Value>) -> DefinitionValue {
        DefinitionValue {
            definition: definition.version(),
            value,
        }
    }

    pub fn definition(&self) -> &String {
        return &self.definition;
    }

    pub fn value(&self) -> &Map<String, Value> {
        return &self.value;
    }


}
