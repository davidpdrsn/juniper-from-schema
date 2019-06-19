//! An AST pass that for each interface finds the types that implement that interface.
//!
//! This information is required to generate the `juniper::graphql_interface!` calls later.

use crate::ast_pass::schema_visitor::SchemaVisitor;
use graphql_parser::schema::*;
use std::collections::HashMap;

pub fn find_interface_implementors(doc: &Document) -> InterfaceImplementors {
    let mut i = InterfaceImplementors::new();
    i.visit_document(doc);
    i
}

#[derive(Debug, Clone)]
pub struct InterfaceImplementors<'doc> {
    map: HashMap<&'doc str, Vec<&'doc str>>,
}

impl InterfaceImplementors<'_> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<&str>> {
        self.map.get(name)
    }
}

impl<'doc> SchemaVisitor<'doc> for InterfaceImplementors<'doc> {
    fn visit_object_type(&mut self, obj: &'doc ObjectType) {
        for interface in &obj.implements_interfaces {
            self.map
                .entry(interface)
                .or_insert_with(Vec::new)
                .push(&obj.name);
        }
    }
}
