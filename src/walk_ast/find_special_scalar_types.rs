use graphql_parser::schema::*;

#[derive(Clone)]
pub struct SpecialScalarTypesList {
    date_defined: bool,
    date_time_defined: bool,
    id_scalar_used: bool,
}

impl SpecialScalarTypesList {
    pub fn date_defined(&self) -> bool {
        self.date_defined
    }

    pub fn date_time_defined(&self) -> bool {
        self.date_time_defined
    }

    pub fn id_scalar_used(&self) -> bool {
        self.id_scalar_used
    }
}

pub fn find_special_scalar_types(doc: &Document) -> SpecialScalarTypesList {
    use graphql_parser::schema::Definition::*;
    use graphql_parser::schema::TypeDefinition::*;

    let mut out = SpecialScalarTypesList {
        date_defined: false,
        date_time_defined: false,
        id_scalar_used: false,
    };

    for def in &doc.definitions {
        match def {
            TypeDefinition(type_def) => match type_def {
                Scalar(scalar_type) => match &*scalar_type.name {
                    "Date" => out.date_defined = true,
                    "DateTime" => out.date_time_defined = true,
                    _ => {}
                },

                Object(obj) => {
                    for field in &obj.fields {
                        if is_id_type(&field.field_type) {
                            out.id_scalar_used = true
                        }

                        for arg in &field.arguments {
                            if is_id_type(&arg.value_type) {
                                out.id_scalar_used = true
                            }
                        }
                    }
                }
                InputObject(obj) => {
                    for field in &obj.fields {
                        if is_id_type(&field.value_type) {
                            out.id_scalar_used = true
                        }
                    }
                }

                _ => {}
            },
            _ => {}
        }
    }

    out
}

fn is_id_type(r#type: &Type) -> bool {
    use graphql_parser::query::Type::*;

    match r#type {
        NamedType(name) => name == "ID",
        ListType(inner) => is_id_type(&inner),
        NonNullType(inner) => is_id_type(&inner),
    }
}
