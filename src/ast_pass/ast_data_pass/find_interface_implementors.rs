//! An AST pass that for each interface finds the types that implement that interface.
//!
//! This information is required to generate the `juniper::graphql_interface!` calls later.

use graphql_parser::schema::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InterfaceImplementors<'doc> {
    map: HashMap<&'doc str, Vec<&'doc str>>,
}

impl<'doc> InterfaceImplementors<'doc> {
    pub fn get(&self, name: &str) -> Option<&Vec<&str>> {
        self.map.get(name)
    }
}

pub fn find_interface_implementors<'doc>(doc: &'doc Document) -> InterfaceImplementors<'doc> {
    use graphql_parser::schema::Definition::*;
    use graphql_parser::schema::TypeDefinition::*;

    let mut out = InterfaceImplementors {
        map: HashMap::new(),
    };

    for def in &doc.definitions {
        match def {
            TypeDefinition(type_def) => match type_def {
                Object(obj) => {
                    for interface in &obj.implements_interfaces {
                        out.map
                            .entry(interface)
                            .and_modify(|entry| entry.push(&obj.name))
                            .or_insert(vec![&obj.name]);
                    }
                }

                _ => {}
            },
            _ => {}
        }
    }

    out
}
