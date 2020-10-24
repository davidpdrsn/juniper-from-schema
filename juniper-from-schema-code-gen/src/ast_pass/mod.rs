pub mod code_gen_pass;
pub mod directive_parsing;
pub mod error;
pub mod schema_visitor;
pub mod validations;

use self::{
    directive_parsing::{DateTimeScalarType, ParseDirective},
    error::{Error, ErrorKind},
    schema_visitor::{visit_document, SchemaVisitor},
};
use graphql_parser::schema::*;
use graphql_parser::Pos;
use std::collections::{BTreeSet, HashMap, HashSet};

pub fn type_name<'doc>(type_: &Type<'doc, &'doc str>) -> &'doc str {
    match &*type_ {
        Type::NamedType(name) => &name,
        Type::ListType(item_type) => type_name(&*item_type),
        Type::NonNullType(item_type) => type_name(&*item_type),
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
    Scalar,
    Type,
}

pub trait EmitError {
    fn emit_error(&mut self, pos: Pos, kind: ErrorKind);
}

impl EmitError for BTreeSet<Error> {
    fn emit_error(&mut self, pos: Pos, kind: ErrorKind) {
        let error = Error { pos, kind };
        self.insert(error);
    }
}

#[derive(Debug)]
pub struct AstData<'doc> {
    interface_implementors: HashMap<&'doc str, Vec<&'doc str>>,
    user_scalars: HashSet<&'doc str>,
    enum_types: HashSet<&'doc str>,
    union_types: HashSet<&'doc str>,
    input_object_field_types: HashMap<&'doc str, HashMap<&'doc str, &'doc Type<'doc, &'doc str>>>,
    errors: BTreeSet<Error>,
    include_time_zone_on_date_time_scalar: bool,
    subscription_type_name: Option<&'doc str>,
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

    fn visit_schema_definition(&mut self, node: &'doc SchemaDefinition<'doc, &'doc str>) {
        if let Some(subscription_type_name) = node.subscription {
            self.subscription_type_name = Some(subscription_type_name);
        }
    }
}

impl<'doc> AstData<'doc> {
    pub fn new_from_doc(doc: &'doc Document<'doc, &'doc str>) -> Result<Self, BTreeSet<Error>> {
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
            subscription_type_name: None,
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

    pub fn input_object_field_type_name(
        &self,
        input_type_name: &'doc str,
        field_name: &'doc str,
    ) -> Option<&'doc str> {
        let field_map = &self.input_object_field_types.get(input_type_name)?;
        let type_ = field_map.get(field_name)?;
        Some(type_name(&type_))
    }

    pub fn is_subscription_type(&self, name: &'doc str) -> bool {
        self.subscription_type_name
            .map(|s| s == name)
            .unwrap_or(false)
    }
}

impl<'doc> EmitError for AstData<'doc> {
    fn emit_error(&mut self, pos: Pos, kind: ErrorKind) {
        let error = Error { pos, kind };
        self.errors.insert(error);
    }
}

pub enum DateTimeScalarDefinition {
    WithTimeZone,
    WithoutTimeZone,
}

#[derive(Eq, PartialEq, Debug)]
pub enum NullableType<'a> {
    NamedType(&'a str),
    ListType(Box<NullableType<'a>>),
    NullableType(Box<NullableType<'a>>),
}

impl<'a> NullableType<'a> {
    pub fn from_schema_type(ty: &Type<'a, &'a str>) -> Self {
        map(&ty)
    }
}

#[cfg(test)]
impl<'a> NullableType<'a> {
    fn debug_print(&self) -> String {
        match self {
            NullableType::NamedType(name) => (*name).to_string(),
            NullableType::ListType(inner) => format!("List({})", inner.debug_print()),
            NullableType::NullableType(inner) => format!("Nullable({})", inner.debug_print()),
        }
    }
}

fn map<'a>(ty: &Type<'a, &'a str>) -> NullableType<'a> {
    match ty {
        inner @ Type::NamedType(_) => map_inner(inner, false),
        Type::ListType(item_type) => {
            let item_type = map_inner(&*item_type, false);
            let list = NullableType::ListType(Box::new(item_type));
            NullableType::NullableType(Box::new(list))
        }
        Type::NonNullType(inner) => map_inner(&*inner, true),
    }
}

fn map_inner<'a>(ty: &Type<'a, &'a str>, inside_non_null: bool) -> NullableType<'a> {
    match ty {
        Type::NamedType(name) => {
            let inner_mapped = NullableType::NamedType(&name);
            if inside_non_null {
                inner_mapped
            } else {
                NullableType::NullableType(Box::new(inner_mapped))
            }
        }
        Type::ListType(inner) => {
            let inner_mapped = NullableType::ListType(Box::new(map(&*inner)));
            if inside_non_null {
                inner_mapped
            } else {
                NullableType::NullableType(Box::new(inner_mapped))
            }
        }
        Type::NonNullType(inner) => map_inner(&*inner, true),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn named_type() {
        let input = Type::NonNullType(Box::new(Type::NamedType("Int")));
        let expected = "Int".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NamedType("Int");
        let expected = "Nullable(Int)".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NonNullType(
            Box::new(Type::NamedType("Int")),
        )))));
        let expected = "List(Int)".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::ListType(Box::new(Type::NonNullType(Box::new(Type::NamedType(
            "Int",
        )))));
        let expected = "Nullable(List(Int))".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NamedType("Int")))));
        let expected = "List(Nullable(Int))".to_string();
        assert_eq!(map(&input).debug_print(), expected);

        let input = Type::ListType(Box::new(Type::NamedType("Int")));
        let expected = "Nullable(List(Nullable(Int)))".to_string();
        assert_eq!(map(&input).debug_print(), expected);
    }
}
