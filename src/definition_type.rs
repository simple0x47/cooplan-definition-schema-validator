#[derive(Debug, Copy, Clone)]
pub enum DefinitionType {
    Product,
    Modifier,
    Service,
}

impl DefinitionType {
    pub fn attribute_id(&self) -> &str {
        match self {
            DefinitionType::Product => "4ed908eb-50b6-4faa-9baa-a7a897cec30f",
            DefinitionType::Modifier => "d22202c1-44cb-471f-9c69-07f7eea1b9bf",
            DefinitionType::Service => "70b0d023-20e6-45cb-9654-e5fd42749642",
        }
    }
}
