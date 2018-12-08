use graphql_parser::schema::*;

#[derive(Clone)]
pub struct SpecialScalarTypesList {
    date_defined: bool,
    date_time_defined: bool,
}

impl SpecialScalarTypesList {
    pub fn date_defined(&self) -> bool {
        self.date_defined
    }

    pub fn date_time_defined(&self) -> bool {
        self.date_time_defined
    }
}

pub fn find_special_scalar_types(doc: &Document) -> SpecialScalarTypesList {
    use graphql_parser::schema::Definition::*;
    use graphql_parser::schema::TypeDefinition::*;

    let mut out = SpecialScalarTypesList {
        date_defined: false,
        date_time_defined: false,
    };

    for def in &doc.definitions {
        match def {
            TypeDefinition(type_def) => match type_def {
                Scalar(scalar_type) => match &*scalar_type.name {
                    "Date" => out.date_defined = true,
                    "DateTime" => out.date_time_defined = true,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    out
}
