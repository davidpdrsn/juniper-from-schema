use crate::ast_pass::schema_visitor::SchemaVisitor;
use crate::ast_pass::type_name;
use graphql_parser::schema::Document;
use graphql_parser::schema::*;
use std::collections::{HashMap, HashSet};

pub struct AstData<'doc> {
    interface_implementors: InterfaceImplementors<'doc>,
    special_scalars: SpecialScalarTypesList<'doc>,
    enum_variants: EnumVariants<'doc>,
    input_object_field_type: InputObjectFieldTypes<'doc>,
}

impl<'doc> From<&'doc Document> for AstData<'doc> {
    fn from(doc: &'doc Document) -> Self {
        let interface_implementors = InterfaceImplementors::from(doc);
        let special_scalars = SpecialScalarTypesList::from(doc);
        let enum_variants = EnumVariants::from(doc);
        let input_object_field_type = InputObjectFieldTypes::from(doc);

        Self {
            interface_implementors,
            special_scalars,
            enum_variants,
            input_object_field_type,
        }
    }
}

impl<'doc> AstData<'doc> {
    pub fn get_interface_implementor(&self, name: &str) -> Option<&Vec<&str>> {
        self.interface_implementors.get(name)
    }

    pub fn date_scalar_defined(&self) -> bool {
        self.special_scalars.date_defined()
    }

    pub fn date_time_scalar_defined(&self) -> bool {
        self.special_scalars.date_time_defined()
    }

    pub fn is_scalar(&self, name: &str) -> bool {
        self.special_scalars.is_scalar(name)
    }

    pub fn is_enum_variant(&self, name: &str) -> bool {
        self.enum_variants.contains(name)
    }

    pub fn input_object_field_is_nullable(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc String,
    ) -> Option<bool> {
        self.input_object_field_type
            .is_nullable(input_type_name, field_name)
    }

    pub fn input_object_field_names(
        &self,
        input_type_name: &'doc str,
    ) -> Option<HashSet<&'doc &String>> {
        self.input_object_field_type.field_names(input_type_name)
    }

    pub fn input_object_field_type_name(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc String,
    ) -> Option<&'doc Name> {
        self.input_object_field_type
            .field_type_name(input_type_name, field_name)
    }
}

#[derive(Debug, Clone)]
pub struct InputObjectFieldTypes<'a> {
    types: HashMap<&'a str, HashMap<&'a String, &'a Type>>,
}

impl<'a> InputObjectFieldTypes<'a> {
    fn new() -> Self {
        InputObjectFieldTypes {
            types: HashMap::new(),
        }
    }

    #[allow(clippy::ptr_arg)]
    fn is_nullable(&self, input_type_name: &'a str, field_name: &'a String) -> Option<bool> {
        use graphql_parser::query::Type::*;

        let field_map = self.types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        match type_ {
            NamedType(_) => Some(true),
            ListType(_) => Some(true),
            NonNullType(_) => Some(false),
        }
    }

    fn field_names(&self, input_type_name: &'a str) -> Option<HashSet<&'a &String>> {
        let field_map = self.types.get(input_type_name)?;
        let mut out = HashSet::new();
        for key in field_map.keys() {
            out.insert(key);
        }
        Some(out)
    }

    #[allow(clippy::ptr_arg)]
    fn field_type_name(
        &self,
        input_type_name: &'a str,
        field_name: &'a String,
    ) -> Option<&'a Name> {
        let field_map = self.types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        Some(type_name(&type_))
    }
}

impl<'doc> SchemaVisitor<'doc> for InputObjectFieldTypes<'doc> {
    fn visit_input_object_type(&mut self, input_type: &'doc InputObjectType) {
        for field in &input_type.fields {
            self.types
                .entry(&input_type.name)
                .or_insert_with(HashMap::new)
                .insert(&field.name, &field.value_type);
        }
    }
}

impl<'doc> From<&'doc Document> for InputObjectFieldTypes<'doc> {
    fn from(doc: &'doc Document) -> Self {
        let mut i = InputObjectFieldTypes::new();
        i.visit_document(doc);
        i
    }
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

impl<'doc> From<&'doc Document> for InterfaceImplementors<'doc> {
    fn from(doc: &'doc Document) -> Self {
        let mut i = InterfaceImplementors::new();
        i.visit_document(doc);
        i
    }
}

#[derive(Clone)]
pub struct SpecialScalarTypesList<'doc> {
    set: HashSet<&'doc str>,
}

impl<'doc> SpecialScalarTypesList<'doc> {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    fn date_defined(&self) -> bool {
        self.is_scalar("Date")
    }

    fn date_time_defined(&self) -> bool {
        self.is_scalar("DateTime")
    }

    fn is_scalar(&self, name: &str) -> bool {
        self.set.contains(name)
    }
}

impl<'doc> SchemaVisitor<'doc> for SpecialScalarTypesList<'doc> {
    fn visit_scalar_type(&mut self, scalar: &'doc ScalarType) {
        let name = &*scalar.name;
        self.set.insert(name);
    }
}

impl<'doc> From<&'doc Document> for SpecialScalarTypesList<'doc> {
    fn from(doc: &'doc Document) -> Self {
        let mut i = SpecialScalarTypesList::new();
        i.visit_document(doc);
        i
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariants<'doc> {
    set: HashSet<&'doc str>,
}

impl<'doc> EnumVariants<'doc> {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    fn contains(&self, name: &str) -> bool {
        self.set.contains(name)
    }
}

impl<'doc> SchemaVisitor<'doc> for EnumVariants<'doc> {
    fn visit_enum_type(&mut self, enum_type: &'doc EnumType) {
        self.set.insert(&enum_type.name);
    }
}

impl<'doc> From<&'doc Document> for EnumVariants<'doc> {
    fn from(doc: &'doc Document) -> Self {
        let mut i = EnumVariants::new();
        i.visit_document(doc);
        i
    }
}
