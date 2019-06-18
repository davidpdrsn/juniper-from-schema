use graphql_parser::schema;

pub fn visit_document_mut<V: SchemaVisitorMut>(visitor: &mut V, doc: &schema::Document) {
    use graphql_parser::schema::Definition::*;

    for def in &doc.definitions {
        match def {
            SchemaDefinition(inner) => visitor.visit_schema_definition(inner),
            TypeDefinition(inner) => visit_type_definition_mut(visitor, inner),
            TypeExtension(inner) => visit_type_extension_mut(visitor, inner),
            DirectiveDefinition(inner) => visitor.visit_directive_definition(inner),
        }
    }
}

fn visit_type_definition_mut<V: SchemaVisitorMut>(visitor: &mut V, ty: &schema::TypeDefinition) {
    use graphql_parser::schema::TypeDefinition::*;

    match ty {
        Scalar(inner) => visitor.visit_scalar_type(inner),
        Object(inner) => visitor.visit_object_type(inner),
        Interface(inner) => visitor.visit_interface_type(inner),
        Union(inner) => visitor.visit_union_type(inner),
        Enum(inner) => visitor.visit_enum_type(inner),
        InputObject(inner) => visitor.visit_input_object_type(inner),
    }
}

fn visit_type_extension_mut<V: SchemaVisitorMut>(visitor: &mut V, ext: &schema::TypeExtension) {
    use graphql_parser::schema::TypeExtension::*;

    match ext {
        Scalar(inner) => visitor.visit_scalar_type_extension(inner),
        Object(inner) => visitor.visit_object_type_extension(inner),
        Interface(inner) => visitor.visit_interface_type_extension(inner),
        Union(inner) => visitor.visit_union_type_extension(inner),
        Enum(inner) => visitor.visit_enum_type_extension(inner),
        InputObject(inner) => visitor.visit_input_object_type_extension(inner),
    }
}

pub trait SchemaVisitorMut {
    fn visit_schema_definition(&mut self, _: &schema::SchemaDefinition) {}

    fn visit_directive_definition(&mut self, _: &schema::DirectiveDefinition) {}

    fn visit_scalar_type(&mut self, _: &schema::ScalarType) {}
    fn visit_object_type(&mut self, _: &schema::ObjectType) {}
    fn visit_interface_type(&mut self, _: &schema::InterfaceType) {}
    fn visit_union_type(&mut self, _: &schema::UnionType) {}
    fn visit_enum_type(&mut self, _: &schema::EnumType) {}
    fn visit_input_object_type(&mut self, _: &schema::InputObjectType) {}

    fn visit_scalar_type_extension(&mut self, _: &schema::ScalarTypeExtension) {}
    fn visit_object_type_extension(&mut self, _: &schema::ObjectTypeExtension) {}
    fn visit_interface_type_extension(&mut self, _: &schema::InterfaceTypeExtension) {}
    fn visit_union_type_extension(&mut self, _: &schema::UnionTypeExtension) {}
    fn visit_enum_type_extension(&mut self, _: &schema::EnumTypeExtension) {}
    fn visit_input_object_type_extension(&mut self, _: &schema::InputObjectTypeExtension) {}
}
