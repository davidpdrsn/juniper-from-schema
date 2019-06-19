use graphql_parser::schema;

pub trait SchemaVisitor {
    fn visit_document(&mut self, doc: &schema::Document) {
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

    fn visit_schema_definition(&mut self, _: &schema::SchemaDefinition) {}

    fn visit_directive_definition(&mut self, _: &schema::DirectiveDefinition) {}

    fn visit_type_definition(&mut self, ty: &schema::TypeDefinition) {
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
    fn visit_scalar_type(&mut self, _: &schema::ScalarType) {}
    fn visit_object_type(&mut self, _: &schema::ObjectType) {}
    fn visit_interface_type(&mut self, _: &schema::InterfaceType) {}
    fn visit_union_type(&mut self, _: &schema::UnionType) {}
    fn visit_enum_type(&mut self, _: &schema::EnumType) {}
    fn visit_input_object_type(&mut self, _: &schema::InputObjectType) {}

    fn visit_type_extension(&mut self, ext: &schema::TypeExtension) {
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
    fn visit_scalar_type_extension(&mut self, _: &schema::ScalarTypeExtension) {}
    fn visit_object_type_extension(&mut self, _: &schema::ObjectTypeExtension) {}
    fn visit_interface_type_extension(&mut self, _: &schema::InterfaceTypeExtension) {}
    fn visit_union_type_extension(&mut self, _: &schema::UnionTypeExtension) {}
    fn visit_enum_type_extension(&mut self, _: &schema::EnumTypeExtension) {}
    fn visit_input_object_type_extension(&mut self, _: &schema::InputObjectTypeExtension) {}
}
