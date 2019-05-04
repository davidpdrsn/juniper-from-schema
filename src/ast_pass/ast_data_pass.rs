mod find_enum_variants;
mod find_interface_implementors;
mod find_special_scalar_types;

use self::find_enum_variants::{find_enum_variants, EnumVariants};
use self::find_interface_implementors::{find_interface_implementors, InterfaceImplementors};
use self::find_special_scalar_types::{find_special_scalar_types, SpecialScalarTypesList};
use crate::ast_pass::type_name;
use graphql_parser::schema::Document;
use graphql_parser::schema::*;
use std::collections::{HashMap, HashSet};

pub struct AstData<'doc> {
    pub(super) interface_implementors: InterfaceImplementors<'doc>,
    pub(super) special_scalars: SpecialScalarTypesList<'doc>,
    pub(super) enum_variants: EnumVariants<'doc>,
    pub(super) input_object_field_type: InputObjectFieldTypes<'doc>,
}

impl<'doc> AstData<'doc> {
    pub fn new(doc: &'doc Document) -> Self {
        let interface_implementors = find_interface_implementors(&doc);
        let special_scalars = find_special_scalar_types(&doc);
        let enum_variants = find_enum_variants(&doc);
        let input_object_field_type = find_input_object_field_type(&doc);

        Self {
            interface_implementors,
            special_scalars,
            enum_variants,
            input_object_field_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputObjectFieldTypes<'a> {
    types: HashMap<&'a str, HashMap<&'a String, &'a Type>>,
}

impl<'a> InputObjectFieldTypes<'a> {
    pub(super) fn is_nullable(
        &self,
        input_type_name: &'a str,
        field_name: &'a String,
    ) -> Option<bool> {
        use graphql_parser::query::Type::*;

        let field_map = self.types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        match type_ {
            NamedType(_) => Some(true),
            ListType(_) => Some(true),
            NonNullType(_) => Some(false),
        }
    }

    pub(super) fn field_names(&self, input_type_name: &'a str) -> Option<HashSet<&'a &String>> {
        let field_map = self.types.get(input_type_name)?;
        let mut out = HashSet::new();
        for key in field_map.keys() {
            out.insert(key);
        }
        Some(out)
    }

    pub(super) fn field_type_name(
        &self,
        input_type_name: &'a str,
        field_name: &'a String,
    ) -> Option<&'a Name> {
        let field_map = self.types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        Some(type_name(&type_))
    }
}

fn find_input_object_field_type(doc: &Document) -> InputObjectFieldTypes {
    use graphql_parser::schema::Definition::*;
    use graphql_parser::schema::TypeDefinition::*;

    let mut out = InputObjectFieldTypes {
        types: HashMap::new(),
    };

    for def in &doc.definitions {
        match def {
            TypeDefinition(type_def) => match type_def {
                InputObject(input_type) => {
                    for field in &input_type.fields {
                        out.types
                            .entry(&input_type.name)
                            .or_insert_with(HashMap::new)
                            .insert(&field.name, &field.value_type);
                    }
                }

                _ => {}
            },
            _ => {}
        }
    }

    out
}
