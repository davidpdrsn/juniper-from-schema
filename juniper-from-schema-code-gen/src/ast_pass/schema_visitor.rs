use graphql_parser::schema;

pub trait SchemaVisitor<'doc> {
    fn visit_document(&mut self, doc: &'doc schema::Document) {
        use graphql_parser::schema::Definition::*;

        for def in &doc.definitions {
            match def {
                SchemaDefinition(inner) => self.visit_schema_definition(inner),
                TypeDefinition(inner) => self.visit_type_definition(inner),
                TypeExtension(inner) => self.visit_type_extension(inner),
                DirectiveDefinition(inner) => self.visit_directive_definition(inner),
            }
        }
    }

    fn visit_schema_definition(&mut self, _: &'doc schema::SchemaDefinition) {}

    fn visit_directive_definition(&mut self, _: &'doc schema::DirectiveDefinition) {}

    fn visit_type_definition(&mut self, ty: &'doc schema::TypeDefinition) {
        use graphql_parser::schema::TypeDefinition::*;

        match ty {
            Scalar(inner) => self.visit_scalar_type(inner),
            Object(inner) => self.visit_object_type(inner),
            Interface(inner) => self.visit_interface_type(inner),
            Union(inner) => self.visit_union_type(inner),
            Enum(inner) => self.visit_enum_type(inner),
            InputObject(inner) => self.visit_input_object_type(inner),
        }
    }
    fn visit_scalar_type(&mut self, _: &'doc schema::ScalarType) {}
    fn visit_object_type(&mut self, _: &'doc schema::ObjectType) {}
    fn visit_interface_type(&mut self, _: &'doc schema::InterfaceType) {}
    fn visit_union_type(&mut self, _: &'doc schema::UnionType) {}
    fn visit_enum_type(&mut self, _: &'doc schema::EnumType) {}
    fn visit_input_object_type(&mut self, _: &'doc schema::InputObjectType) {}

    fn visit_type_extension(&mut self, ext: &'doc schema::TypeExtension) {
        use graphql_parser::schema::TypeExtension::*;

        match ext {
            Scalar(inner) => self.visit_scalar_type_extension(inner),
            Object(inner) => self.visit_object_type_extension(inner),
            Interface(inner) => self.visit_interface_type_extension(inner),
            Union(inner) => self.visit_union_type_extension(inner),
            Enum(inner) => self.visit_enum_type_extension(inner),
            InputObject(inner) => self.visit_input_object_type_extension(inner),
        }
    }
    fn visit_scalar_type_extension(&mut self, _: &'doc schema::ScalarTypeExtension) {}
    fn visit_object_type_extension(&mut self, _: &'doc schema::ObjectTypeExtension) {}
    fn visit_interface_type_extension(&mut self, _: &'doc schema::InterfaceTypeExtension) {}
    fn visit_union_type_extension(&mut self, _: &'doc schema::UnionTypeExtension) {}
    fn visit_enum_type_extension(&mut self, _: &'doc schema::EnumTypeExtension) {}
    fn visit_input_object_type_extension(&mut self, _: &'doc schema::InputObjectTypeExtension) {}
}
