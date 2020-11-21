#![deny(unused_variables)]

use graphql_parser::{
    schema,
    schema::{Definition, TypeDefinition, TypeExtension},
};

pub trait SchemaVisitor<'doc> {
    #[inline]
    fn visit_document(&mut self, _: &'doc schema::Document<'doc, &'doc str>) {}

    #[inline]
    fn visit_schema_definition(&mut self, _: &'doc schema::SchemaDefinition<'doc, &'doc str>) {}

    #[inline]
    fn visit_directive_definition(
        &mut self,
        _: &'doc schema::DirectiveDefinition<'doc, &'doc str>,
    ) {
    }

    #[inline]
    fn visit_type_definition(&mut self, _: &'doc schema::TypeDefinition<'doc, &'doc str>) {}

    #[inline]
    fn visit_scalar_type(&mut self, _: &'doc schema::ScalarType<'doc, &'doc str>) {}

    #[inline]
    fn visit_object_type(&mut self, _: &'doc schema::ObjectType<'doc, &'doc str>) {}

    #[inline]
    fn visit_interface_type(&mut self, _: &'doc schema::InterfaceType<'doc, &'doc str>) {}

    #[inline]
    fn visit_union_type(&mut self, _: &'doc schema::UnionType<'doc, &'doc str>) {}

    #[inline]
    fn visit_enum_type(&mut self, _: &'doc schema::EnumType<'doc, &'doc str>) {}

    #[inline]
    fn visit_input_object_type(&mut self, _: &'doc schema::InputObjectType<'doc, &'doc str>) {}

    #[inline]
    fn visit_type_extension(&mut self, _: &'doc schema::TypeExtension<'doc, &'doc str>) {}

    #[inline]
    fn visit_scalar_type_extension(
        &mut self,
        _: &'doc schema::ScalarTypeExtension<'doc, &'doc str>,
    ) {
    }

    #[inline]
    fn visit_object_type_extension(
        &mut self,
        _: &'doc schema::ObjectTypeExtension<'doc, &'doc str>,
    ) {
    }

    #[inline]
    fn visit_interface_type_extension(
        &mut self,
        _: &'doc schema::InterfaceTypeExtension<'doc, &'doc str>,
    ) {
    }

    #[inline]
    fn visit_union_type_extension(&mut self, _: &'doc schema::UnionTypeExtension<'doc, &'doc str>) {
    }

    #[inline]
    fn visit_enum_type_extension(&mut self, _: &'doc schema::EnumTypeExtension<'doc, &'doc str>) {}

    #[inline]
    fn visit_input_object_type_extension(
        &mut self,
        _: &'doc schema::InputObjectTypeExtension<'doc, &'doc str>,
    ) {
    }

    #[inline]
    fn and<T>(self, rhs: T) -> And<Self, T>
    where
        Self: Sized,
    {
        And { lhs: self, rhs }
    }
}

pub fn visit_document<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::Document<'doc, &'doc str>,
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
    node: &'doc schema::SchemaDefinition<'doc, &'doc str>,
) {
    v.visit_schema_definition(node)
}

pub fn visit_directive_definition<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::DirectiveDefinition<'doc, &'doc str>,
) {
    v.visit_directive_definition(node)
}

pub fn visit_type_definition<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::TypeDefinition<'doc, &'doc str>,
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
    node: &'doc schema::ScalarType<'doc, &'doc str>,
) {
    v.visit_scalar_type(node)
}

pub fn visit_object_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::ObjectType<'doc, &'doc str>,
) {
    v.visit_object_type(node)
}

pub fn visit_interface_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InterfaceType<'doc, &'doc str>,
) {
    v.visit_interface_type(node)
}

pub fn visit_union_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::UnionType<'doc, &'doc str>,
) {
    v.visit_union_type(node)
}

pub fn visit_enum_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::EnumType<'doc, &'doc str>,
) {
    v.visit_enum_type(node)
}

pub fn visit_input_object_type<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InputObjectType<'doc, &'doc str>,
) {
    v.visit_input_object_type(node)
}

pub fn visit_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::TypeExtension<'doc, &'doc str>,
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
    node: &'doc schema::ScalarTypeExtension<'doc, &'doc str>,
) {
    v.visit_scalar_type_extension(node)
}

pub fn visit_object_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::ObjectTypeExtension<'doc, &'doc str>,
) {
    v.visit_object_type_extension(node)
}

pub fn visit_interface_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InterfaceTypeExtension<'doc, &'doc str>,
) {
    v.visit_interface_type_extension(node)
}

pub fn visit_union_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::UnionTypeExtension<'doc, &'doc str>,
) {
    v.visit_union_type_extension(node)
}

pub fn visit_enum_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::EnumTypeExtension<'doc, &'doc str>,
) {
    v.visit_enum_type_extension(node)
}

pub fn visit_input_object_type_extension<'doc, V: SchemaVisitor<'doc>>(
    v: &mut V,
    node: &'doc schema::InputObjectTypeExtension<'doc, &'doc str>,
) {
    v.visit_input_object_type_extension(node)
}

#[derive(Debug)]
pub struct And<A, B> {
    lhs: A,
    rhs: B,
}

impl<A, B> And<A, B> {
    pub fn into_inner(self) -> (A, B) {
        (self.lhs, self.rhs)
    }
}

impl<'doc, A, B> SchemaVisitor<'doc> for And<A, B>
where
    A: SchemaVisitor<'doc>,
    B: SchemaVisitor<'doc>,
{
    #[inline]
    fn visit_document(&mut self, node: &'doc schema::Document<'doc, &'doc str>) {
        self.lhs.visit_document(node);
        self.rhs.visit_document(node);
    }

    #[inline]
    fn visit_schema_definition(&mut self, node: &'doc schema::SchemaDefinition<'doc, &'doc str>) {
        self.lhs.visit_schema_definition(node);
        self.rhs.visit_schema_definition(node);
    }

    #[inline]
    fn visit_directive_definition(
        &mut self,
        node: &'doc schema::DirectiveDefinition<'doc, &'doc str>,
    ) {
        self.lhs.visit_directive_definition(node);
        self.rhs.visit_directive_definition(node);
    }

    #[inline]
    fn visit_type_definition(&mut self, node: &'doc schema::TypeDefinition<'doc, &'doc str>) {
        self.lhs.visit_type_definition(node);
        self.rhs.visit_type_definition(node);
    }

    #[inline]
    fn visit_scalar_type(&mut self, node: &'doc schema::ScalarType<'doc, &'doc str>) {
        self.lhs.visit_scalar_type(node);
        self.rhs.visit_scalar_type(node);
    }

    #[inline]
    fn visit_object_type(&mut self, node: &'doc schema::ObjectType<'doc, &'doc str>) {
        self.lhs.visit_object_type(node);
        self.rhs.visit_object_type(node);
    }

    #[inline]
    fn visit_interface_type(&mut self, node: &'doc schema::InterfaceType<'doc, &'doc str>) {
        self.lhs.visit_interface_type(node);
        self.rhs.visit_interface_type(node);
    }

    #[inline]
    fn visit_union_type(&mut self, node: &'doc schema::UnionType<'doc, &'doc str>) {
        self.lhs.visit_union_type(node);
        self.rhs.visit_union_type(node);
    }

    #[inline]
    fn visit_enum_type(&mut self, node: &'doc schema::EnumType<'doc, &'doc str>) {
        self.lhs.visit_enum_type(node);
        self.rhs.visit_enum_type(node);
    }

    #[inline]
    fn visit_input_object_type(&mut self, node: &'doc schema::InputObjectType<'doc, &'doc str>) {
        self.lhs.visit_input_object_type(node);
        self.rhs.visit_input_object_type(node);
    }

    #[inline]
    fn visit_type_extension(&mut self, node: &'doc schema::TypeExtension<'doc, &'doc str>) {
        self.lhs.visit_type_extension(node);
        self.rhs.visit_type_extension(node);
    }

    #[inline]
    fn visit_scalar_type_extension(
        &mut self,
        node: &'doc schema::ScalarTypeExtension<'doc, &'doc str>,
    ) {
        self.lhs.visit_scalar_type_extension(node);
        self.rhs.visit_scalar_type_extension(node);
    }

    #[inline]
    fn visit_object_type_extension(
        &mut self,
        node: &'doc schema::ObjectTypeExtension<'doc, &'doc str>,
    ) {
        self.lhs.visit_object_type_extension(node);
        self.rhs.visit_object_type_extension(node);
    }

    #[inline]
    fn visit_interface_type_extension(
        &mut self,
        node: &'doc schema::InterfaceTypeExtension<'doc, &'doc str>,
    ) {
        self.lhs.visit_interface_type_extension(node);
        self.rhs.visit_interface_type_extension(node);
    }

    #[inline]
    fn visit_union_type_extension(
        &mut self,
        node: &'doc schema::UnionTypeExtension<'doc, &'doc str>,
    ) {
        self.lhs.visit_union_type_extension(node);
        self.rhs.visit_union_type_extension(node);
    }

    #[inline]
    fn visit_enum_type_extension(
        &mut self,
        node: &'doc schema::EnumTypeExtension<'doc, &'doc str>,
    ) {
        self.lhs.visit_enum_type_extension(node);
        self.rhs.visit_enum_type_extension(node);
    }

    #[inline]
    fn visit_input_object_type_extension(
        &mut self,
        node: &'doc schema::InputObjectTypeExtension<'doc, &'doc str>,
    ) {
        self.lhs.visit_input_object_type_extension(node);
        self.rhs.visit_input_object_type_extension(node);
    }
}
