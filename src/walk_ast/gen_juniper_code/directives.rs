#![allow(unused_imports, dead_code)]

use graphql_parser::schema;

pub trait GetDirectives {
    fn directives(&self) -> &Vec<schema::Directive>;
}

pub fn panic_if_has_directives<T: GetDirectives>(t: &T) {
    if !t.directives().is_empty() {
        todo!("Directives are not supported yet");
    }
}

macro_rules! impl_GetDirectives {
    ($name:path) => {
        impl GetDirectives for $name {
            fn directives(&self) -> &Vec<schema::Directive> {
                &self.directives
            }
        }
    };
}

// All the schema types that have directives
impl_GetDirectives!(schema::EnumType);
impl_GetDirectives!(schema::EnumValue);
impl_GetDirectives!(schema::Field);
impl_GetDirectives!(schema::InputObjectType);
impl_GetDirectives!(schema::InputValue);
impl_GetDirectives!(schema::InterfaceType);
impl_GetDirectives!(schema::ObjectType);
impl_GetDirectives!(schema::ScalarType);
impl_GetDirectives!(schema::SchemaDefinition);
impl_GetDirectives!(schema::UnionType);
// Not supported yet
// impl_GetDirectives!(schema::EnumTypeExtension);
// impl_GetDirectives!(schema::InterfaceTypeExtension);
// impl_GetDirectives!(schema::ObjectTypeExtension);
// impl_GetDirectives!(schema::ScalarTypeExtension);
// impl_GetDirectives!(schema::UnionTypeExtension);
