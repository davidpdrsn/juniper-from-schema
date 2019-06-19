use crate::ast_pass::schema_visitor::SchemaVisitor;
use crate::ast_pass::type_name;
use graphql_parser::schema::Document;
use graphql_parser::schema::*;
use std::collections::{HashMap, HashSet};

pub struct AstData<'doc> {
    interface_implementors: HashMap<&'doc str, Vec<&'doc str>>,
    special_scalars: HashSet<&'doc str>,
    enum_variants: HashSet<&'doc str>,
    input_object_field_types: HashMap<&'doc str, HashMap<&'doc String, &'doc Type>>,
}

impl<'doc> From<&'doc Document> for AstData<'doc> {
    fn from(doc: &'doc Document) -> Self {
        let mut data = AstData::new();
        data.visit_document(doc);
        data
    }
}

impl<'doc> SchemaVisitor<'doc> for AstData<'doc> {
    fn visit_object_type(&mut self, obj: &'doc ObjectType) {
        for interface in &obj.implements_interfaces {
            self.interface_implementors
                .entry(interface)
                .or_insert_with(Vec::new)
                .push(&obj.name);
        }
    }

    fn visit_scalar_type(&mut self, scalar: &'doc ScalarType) {
        let name = &*scalar.name;
        self.special_scalars.insert(name);
    }

    fn visit_enum_type(&mut self, enum_type: &'doc EnumType) {
        self.enum_variants.insert(&enum_type.name);
    }

    fn visit_input_object_type(&mut self, input_type: &'doc InputObjectType) {
        for field in &input_type.fields {
            self.input_object_field_types
                .entry(&input_type.name)
                .or_insert_with(HashMap::new)
                .insert(&field.name, &field.value_type);
        }
    }
}

impl<'doc> AstData<'doc> {
    fn new() -> Self {
        Self {
            interface_implementors: HashMap::new(),
            special_scalars: HashSet::new(),
            enum_variants: HashSet::new(),
            input_object_field_types: HashMap::new(),
        }
    }

    pub fn get_interface_implementor(&self, name: &str) -> Option<&Vec<&str>> {
        self.interface_implementors.get(name)
    }

    pub fn date_scalar_defined(&self) -> bool {
        self.is_scalar("Date")
    }

    pub fn date_time_scalar_defined(&self) -> bool {
        self.is_scalar("DateTime")
    }

    pub fn is_scalar(&self, name: &str) -> bool {
        self.special_scalars.contains(name)
    }

    pub fn is_enum_variant(&self, name: &str) -> bool {
        self.enum_variants.contains(name)
    }

    #[allow(clippy::ptr_arg)]
    pub fn input_object_field_is_nullable(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc String,
    ) -> Option<bool> {
        use graphql_parser::query::Type::*;

        let field_map = self.input_object_field_types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        match type_ {
            NamedType(_) => Some(true),
            ListType(_) => Some(true),
            NonNullType(_) => Some(false),
        }
    }

    pub fn input_object_field_names(
        &self,
        input_type_name: &'doc str,
    ) -> Option<HashSet<&'doc &String>> {
        let field_map = self.input_object_field_types.get(input_type_name)?;
        let mut out = HashSet::new();
        for key in field_map.keys() {
            out.insert(key);
        }
        Some(out)
    }

    #[allow(clippy::ptr_arg)]
    pub fn input_object_field_type_name(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc String,
    ) -> Option<&'doc Name> {
        let field_map = self.input_object_field_types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        Some(type_name(&type_))
    }
}
