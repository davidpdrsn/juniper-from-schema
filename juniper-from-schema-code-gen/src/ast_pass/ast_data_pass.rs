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
    enum_variants: HashSet<&'doc str>,
    input_object_field_types: HashMap<&'doc str, HashMap<&'doc String, &'doc Type>>,
    errors: BTreeSet<Error<'doc>>,
    raw_schema: &'doc str,
    include_time_zone_on_date_time_scalar: bool,
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
        match &*scalar.name {
            name @ "DateTime" => {
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
    pub fn new_from_schema_and_doc(
        raw_schema: &'doc str,
        doc: &'doc Document,
    ) -> Result<Self, BTreeSet<Error<'doc>>> {
        let mut data = Self::new(raw_schema);
        data.visit_document(doc);

        if data.errors.is_empty() {
            Ok(data)
        } else {
            Err(data.errors)
        }
    }

    fn new(raw_schema: &'doc str) -> Self {
        Self {
            interface_implementors: Default::default(),
            user_scalars: Default::default(),
            enum_variants: Default::default(),
            input_object_field_types: Default::default(),
            errors: Default::default(),
            raw_schema,
            include_time_zone_on_date_time_scalar: true,
        }
    }

    pub fn get_implementors_of_interface(&self, name: &str) -> Option<&Vec<&str>> {
        self.interface_implementors.get(name)
    }

    pub fn date_scalar_defined(&self) -> bool {
        self.is_scalar("Date")
    }

    pub fn date_time_scalar_defined(&self) -> bool {
        self.is_scalar("DateTime")
    }

    pub fn date_time_scalar_definition(&self) -> Option<DateTimeScalarDefinition> {
        if self.is_scalar("DateTime") {
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
        self.is_scalar("Uuid")
    }

    pub fn url_scalar_defined(&self) -> bool {
        self.is_scalar("Url")
    }

    pub fn is_scalar(&self, name: &str) -> bool {
        self.user_scalars.contains(name)
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

impl<'doc> EmitError<'doc> for AstData<'doc> {
    fn emit_non_fatal_error(&mut self, pos: Pos, kind: ErrorKind<'doc>) {
        let error = Error {
            pos,
            kind,
            raw_schema: &self.raw_schema,
        };
        self.errors.insert(error);
    }
}

pub enum DateTimeScalarDefinition {
    WithTimeZone,
    WithoutTimeZone,
}
