#![deny(unused_variables)]

use graphql_parser::schema;
use graphql_parser::schema::Definition;
use graphql_parser::schema::TypeDefinition;
use graphql_parser::schema::TypeExtension;

pub trait SchemaVisitor<'doc> {
    #[inline]
    fn visit_document(&mut self, _: &'doc schema::Document<&'doc str>) {}

    #[inline]
    fn visit_schema_definition(&mut self, _: &'doc schema::SchemaDefinition<&'doc str>) {}

    #[inline]
    fn visit_directive_definition(&mut self, _: &'doc schema::DirectiveDefinition<&'doc str>) {}

    #[inline]
    fn visit_type_definition(&mut self, _: &'doc schema::TypeDefinition<&'doc str>) {}

    #[inline]
    fn visit_scalar_type(&mut self, _: &'doc schema::ScalarType<&'doc str>) {}

    #[inline]
    fn visit_object_type(&mut self, _: &'doc schema::ObjectType<&'doc str>) {}

    #[inline]
    fn visit_interface_type(&mut self, _: &'doc schema::InterfaceType<&'doc str>) {}

    #[inline]
    fn visit_union_type(&mut self, _: &'doc schema::UnionType<&'doc str>) {}

    #[inline]
    fn visit_enum_type(&mut self, _: &'doc schema::EnumType<&'doc str>) {}

    #[inline]
    fn visit_input_object_type(&mut self, _: &'doc schema::InputObjectType<&'doc str>) {}

    #[inline]
    fn visit_type_extension(&mut self, _: &'doc schema::TypeExtension<&'doc str>) {}

    #[inline]
    fn visit_scalar_type_extension(&mut self, _: &'doc schema::ScalarTypeExtension<&'doc str>) {}

    #[inline]
    fn visit_object_type_extension(&mut self, _: &'doc schema::ObjectTypeExtension<&'doc str>) {}

    #[inline]
    fn visit_interface_type_extension(
        &mut self,
        _: &'doc schema::InterfaceTypeExtension<&'doc str>,
    ) {
    }

    #[inline]
    fn visit_union_type_extension(&mut self, _: &'doc schema::UnionTypeExtension<&'doc str>) {}

    #[inline]
    fn visit_enum_type_extension(&mut self, _: &'doc schema::EnumTypeExtension<&'doc str>) {}

    #[inline]
    fn visit_input_object_type_extension(
        &mut self,
        _: &'doc schema::InputObjectTypeExtension<&'doc str>,
    ) {
    }
}

pub fn visit_document<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::Document<&'doc str>,
) {
    v.visit_document(node);

    for def in &node.definitions {
        match def {
            Definition::SchemaDefinition(inner) => visit_schema_definition(v, inner),
            Definition::TypeDefinition(inner) => visit_type_definition(v, inner),
            Definition::TypeExtension(inner) => visit_type_extension(v, inner),
            Definition::DirectiveDefinition(inner) => visit_directive_definition(v, inner),
        }
    }
}

pub fn visit_schema_definition<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::SchemaDefinition<&'doc str>,
) {
    v.visit_schema_definition(node)
}

pub fn visit_directive_definition<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::DirectiveDefinition<&'doc str>,
) {
    v.visit_directive_definition(node)
}

pub fn visit_type_definition<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::TypeDefinition<&'doc str>,
) {
    v.visit_type_definition(node);
    match node {
        TypeDefinition::Scalar(inner) => visit_scalar_type(v, inner),
        TypeDefinition::Object(inner) => visit_object_type(v, inner),
        TypeDefinition::Interface(inner) => visit_interface_type(v, inner),
        TypeDefinition::Union(inner) => visit_union_type(v, inner),
        TypeDefinition::Enum(inner) => visit_enum_type(v, inner),
        TypeDefinition::InputObject(inner) => visit_input_object_type(v, inner),
    }
}

pub fn visit_scalar_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::ScalarType<&'doc str>,
) {
    v.visit_scalar_type(node)
}

pub fn visit_object_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::ObjectType<&'doc str>,
) {
    v.visit_object_type(node)
}

pub fn visit_interface_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InterfaceType<&'doc str>,
) {
    v.visit_interface_type(node)
}

pub fn visit_union_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::UnionType<&'doc str>,
) {
    v.visit_union_type(node)
}

pub fn visit_enum_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::EnumType<&'doc str>,
) {
    v.visit_enum_type(node)
}

pub fn visit_input_object_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InputObjectType<&'doc str>,
) {
    v.visit_input_object_type(node)
}

pub fn visit_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::TypeExtension<&'doc str>,
) {
    v.visit_type_extension(node);
    match node {
        TypeExtension::Scalar(inner) => visit_scalar_type_extension(v, inner),
        TypeExtension::Object(inner) => visit_object_type_extension(v, inner),
        TypeExtension::Interface(inner) => visit_interface_type_extension(v, inner),
        TypeExtension::Union(inner) => visit_union_type_extension(v, inner),
        TypeExtension::Enum(inner) => visit_enum_type_extension(v, inner),
        TypeExtension::InputObject(inner) => visit_input_object_type_extension(v, inner),
    }
}

pub fn visit_scalar_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::ScalarTypeExtension<&'doc str>,
) {
    v.visit_scalar_type_extension(node)
}

pub fn visit_object_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::ObjectTypeExtension<&'doc str>,
) {
    v.visit_object_type_extension(node)
}

pub fn visit_interface_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InterfaceTypeExtension<&'doc str>,
) {
    v.visit_interface_type_extension(node)
}

pub fn visit_union_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::UnionTypeExtension<&'doc str>,
) {
    v.visit_union_type_extension(node)
}

pub fn visit_enum_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::EnumTypeExtension<&'doc str>,
) {
    v.visit_enum_type_extension(node)
}

pub fn visit_input_object_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InputObjectTypeExtension<&'doc str>,
) {
    v.visit_input_object_type_extension(node)
}
