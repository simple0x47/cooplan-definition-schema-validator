use crate::category_chain::CategoryChain;
use crate::definition_type::DefinitionType;
use crate::error::{Error, ErrorKind};
use cooplan_definitions_lib::definition::Definition;
use serde_json::{Map, Value};

/// Validated, definition scoped, JSON value.
#[derive(Debug)]
pub struct DefinitionValue {
    definition: String,
    definition_type: DefinitionType,
    category_chain: CategoryChain,
    value: Map<String, Value>,
}

impl DefinitionValue {
    pub fn try_new(
        definition: &Definition,
        category_chain: CategoryChain,
        value: Map<String, Value>,
    ) -> Result<DefinitionValue, Error> {
        let definition_type = if value.contains_key(DefinitionType::Product.attribute_id()) {
            DefinitionType::Product
        } else if value.contains_key(DefinitionType::Modifier.attribute_id()) {
            DefinitionType::Modifier
        } else if value.contains_key(DefinitionType::Service.attribute_id()) {
            DefinitionType::Service
        } else {
            return Err(Error::new(
                ErrorKind::InvalidValue,
                "definition type not found",
            ));
        };

        Ok(DefinitionValue {
            definition: definition.version(),
            category_chain,
            definition_type,
            value,
        })
    }

    pub fn definition(&self) -> &String {
        &self.definition
    }

    pub fn value(&self) -> &Map<String, Value> {
        &self.value
    }

    pub fn category_chain(&self) -> &CategoryChain {
        &self.category_chain
    }

    pub fn definition_type(&self) -> DefinitionType {
        self.definition_type
    }
}
