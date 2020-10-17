use super::schema_visitor::visit_document;
use crate::ast_pass::{
    directive_parsing::{DateTimeScalarType, ParseDirective},
    error::{Error, ErrorKind},
    schema_visitor::SchemaVisitor,
    type_name, EmitError,
};
use graphql_parser::{
    schema::{Document, *},
    Pos,
};
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug)]
pub struct AstData<'doc> {
    interface_implementors: HashMap<&'doc str, Vec<&'doc str>>,
    user_scalars: HashSet<&'doc str>,
    enum_types: HashSet<&'doc str>,
    union_types: HashSet<&'doc str>,
    input_object_field_types: HashMap<&'doc str, HashMap<&'doc str, &'doc Type<'doc, &'doc str>>>,
    errors: BTreeSet<Error<'doc>>,
    include_time_zone_on_date_time_scalar: bool,
}

impl<'doc> SchemaVisitor<'doc> for AstData<'doc> {
    fn visit_object_type(&mut self, obj: &'doc ObjectType<'doc, &'doc str>) {
        for interface in &obj.implements_interfaces {
            self.interface_implementors
                .entry(interface)
                .or_insert_with(Vec::new)
                .push(&obj.name);
        }
    }

    fn visit_scalar_type(&mut self, scalar: &'doc ScalarType<'doc, &'doc str>) {
        match &*scalar.name {
            name if name == crate::DATE_TIME_SCALAR_NAME => {
                let args = self.parse_directives(DateTimeScalarType(scalar));
                if args.with_time_zone {
                    self.include_time_zone_on_date_time_scalar = true;
                } else {
                    self.include_time_zone_on_date_time_scalar = false;
                }
                self.user_scalars.insert(name);
            }
            name => {
                self.user_scalars.insert(name);
            }
        };
    }

    fn visit_enum_type(&mut self, enum_type: &'doc EnumType<'doc, &'doc str>) {
        self.enum_types.insert(&enum_type.name);
    }

    fn visit_union_type(&mut self, union_type: &'doc UnionType<'doc, &'doc str>) {
        self.union_types.insert(&union_type.name);
    }

    fn visit_input_object_type(&mut self, input_type: &'doc InputObjectType<'doc, &'doc str>) {
        for field in &input_type.fields {
            self.input_object_field_types
                .entry(&input_type.name)
                .or_insert_with(HashMap::new)
                .insert(&field.name, &field.value_type);
        }
    }
}

impl<'doc> AstData<'doc> {
    pub fn new_from_doc(
        doc: &'doc Document<'doc, &'doc str>,
    ) -> Result<Self, BTreeSet<Error<'doc>>> {
        let mut data = Self::new();
        visit_document(&mut data, doc);

        if data.errors.is_empty() {
            Ok(data)
        } else {
            Err(data.errors)
        }
    }

    fn new() -> Self {
        Self {
            interface_implementors: Default::default(),
            user_scalars: Default::default(),
            enum_types: Default::default(),
            union_types: Default::default(),
            input_object_field_types: Default::default(),
            errors: Default::default(),
            include_time_zone_on_date_time_scalar: true,
        }
    }

    pub fn get_implementors_of_interface(&self, name: &str) -> Option<&Vec<&'doc str>> {
        self.interface_implementors.get(name)
    }

    pub fn date_scalar_defined(&self) -> bool {
        self.is_scalar(crate::DATE_SCALAR_NAME)
    }

    pub fn date_time_scalar_defined(&self) -> bool {
        self.is_scalar(crate::DATE_TIME_SCALAR_NAME)
    }

    pub fn date_time_scalar_definition(&self) -> Option<DateTimeScalarDefinition> {
        if self.is_scalar(crate::DATE_TIME_SCALAR_NAME) {
            if self.include_time_zone_on_date_time_scalar {
                Some(DateTimeScalarDefinition::WithTimeZone)
            } else {
                Some(DateTimeScalarDefinition::WithoutTimeZone)
            }
        } else {
            None
        }
    }

    pub fn uuid_scalar_defined(&self) -> bool {
        self.is_scalar(crate::UUID_SCALAR_NAME)
    }

    pub fn url_scalar_defined(&self) -> bool {
        self.is_scalar(crate::URL_SCALAR_NAME)
    }

    pub fn is_scalar(&self, name: &str) -> bool {
        self.user_scalars.contains(name)
    }

    pub fn is_enum_type(&self, name: &str) -> bool {
        self.enum_types.contains(name)
    }

    pub fn is_union_type(&self, name: &str) -> bool {
        self.union_types.contains(name)
    }

    pub fn is_interface_type(&self, name: &str) -> bool {
        self.interface_implementors.contains_key(name)
    }

    #[allow(clippy::ptr_arg)]
    pub fn input_object_field_is_nullable(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc str,
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
    ) -> Option<HashSet<&'doc str>> {
        let field_map = self.input_object_field_types.get(input_type_name)?;
        let mut out = HashSet::new();
        for key in field_map.keys() {
            out.insert(*key);
        }
        Some(out)
    }

    #[allow(clippy::ptr_arg)]
    pub fn input_object_field_type_name(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc str,
    ) -> Option<&'doc str> {
        let field_map = &self.input_object_field_types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        Some(type_name(&type_))
    }
}

impl<'doc> EmitError<'doc> for AstData<'doc> {
    fn emit_error(&mut self, pos: Pos, kind: ErrorKind<'doc>) {
        let error = Error { pos, kind };
        self.errors.insert(error);
    }
}

pub enum DateTimeScalarDefinition {
    WithTimeZone,
    WithoutTimeZone,
}
