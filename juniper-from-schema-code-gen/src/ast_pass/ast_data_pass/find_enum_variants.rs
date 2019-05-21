use graphql_parser::schema::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct EnumVariants<'doc> {
    set: HashSet<&'doc str>,
}

impl<'doc> EnumVariants<'doc> {
    pub fn contains(&self, name: &str) -> bool {
        self.set.contains(name)
    }
}

#[allow(clippy::single_match)]
pub fn find_enum_variants(doc: &Document) -> EnumVariants {
    use graphql_parser::schema::Definition::*;
    use graphql_parser::schema::TypeDefinition::*;

    let mut out = EnumVariants {
        set: HashSet::new(),
    };

    for def in &doc.definitions {
        match def {
            TypeDefinition(type_def) => match type_def {
                Enum(enum_type) => {
                    out.set.insert(&enum_type.name);
                }

                _ => {}
            },
            _ => {}
        }
    }

    out
}
