use cooplan_definitions_lib::definition::Definition;
use serde_json::Value;

/// Validated, definition scoped, JSON value.
#[derive(Debug)]
pub struct DefinitionValue {
    definition: Definition,
    value: Value,
}

impl DefinitionValue {
    pub fn new(definition: &Definition, value: Value) -> DefinitionValue {
        DefinitionValue {
            definition: definition.clone(),
            value,
        }
    }

    pub fn definition(&self) -> &Definition {
        return &self.definition;
    }

    pub fn value(&self) -> &Value {
        return &self.value;
    }
}
