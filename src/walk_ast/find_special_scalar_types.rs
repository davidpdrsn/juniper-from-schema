use graphql_parser::schema::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct SpecialScalarTypesList {
    custom_scalars: HashSet<String>,
}

impl SpecialScalarTypesList {
    pub fn date_defined(&self) -> bool {
        self.is_scalar("Date")
    }

    pub fn date_time_defined(&self) -> bool {
        self.is_scalar("DateTime")
    }

    pub fn is_scalar(&self, name: &str) -> bool {
        self.custom_scalars.contains(name)
    }
}

pub fn find_special_scalar_types(doc: &Document) -> SpecialScalarTypesList {
    use graphql_parser::schema::Definition::*;
    use graphql_parser::schema::TypeDefinition::*;

    let mut out = SpecialScalarTypesList {
        custom_scalars: HashSet::new(),
    };

    for def in &doc.definitions {
        match def {
            TypeDefinition(type_def) => match type_def {
                Scalar(scalar_type) => {
                    let name = &*scalar_type.name;
                    out.custom_scalars.insert(name.to_string());
                }

                _ => {}
            },
            _ => {}
        }
    }

    out
}
