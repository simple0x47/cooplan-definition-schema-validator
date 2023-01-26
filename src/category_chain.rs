use cooplan_definitions_lib::definition::Definition;

#[derive(Debug)]
pub struct CategoryChain {
    category_chain: Vec<String>,
}

impl CategoryChain {
    pub fn new(category_chain: Vec<String>) -> CategoryChain {
        CategoryChain { category_chain }
    }

    pub fn contains(&self, category: &String) -> bool {
        self.category_chain.contains(category)
    }
}

pub fn build_from_definition_and_category(
    definition: &Definition,
    category: &String,
) -> CategoryChain {
    let mut category_chain = Vec::new();

    add_category_to_chain(definition, category, &mut category_chain);

    CategoryChain::new(category_chain)
}

fn add_category_to_chain(
    definition: &Definition,
    category: &String,
    category_chain: &mut Vec<String>,
) {
    for definition_category in definition.categories() {
        if &definition_category.id == category {
            if let Some(parent) = definition_category.parent {
                add_category_to_chain(definition, &parent, category_chain);
            }

            category_chain.push(definition_category.id);
            break;
        }
    }
}
