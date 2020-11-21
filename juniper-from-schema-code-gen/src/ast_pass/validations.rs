use std::collections::BTreeSet;

use super::{error::Error, schema_visitor::SchemaVisitor, EmitError, ErrorKind};
use graphql_parser::{
    schema::{self, *},
    Pos,
};
use heck::SnakeCase;

pub struct FieldNameCaseValidator {
    pub errors: BTreeSet<Error>,
}

impl FieldNameCaseValidator {
    pub fn new() -> Self {
        Self {
            errors: Default::default(),
        }
    }
}

impl<'doc> SchemaVisitor<'doc> for FieldNameCaseValidator {
    fn visit_object_type(&mut self, ty: &'doc schema::ObjectType<'doc, &'doc str>) {
        self.validate_fields(&ty.fields);
    }

    fn visit_interface_type(&mut self, ty: &'doc schema::InterfaceType<'doc, &'doc str>) {
        self.validate_fields(&ty.fields);
    }

    fn visit_input_object_type(&mut self, ty: &'doc schema::InputObjectType<'doc, &'doc str>) {
        for field in &ty.fields {
            self.validate_field(&field.name, field.position);
        }
    }
}

impl FieldNameCaseValidator {
    fn validate_fields<'doc>(&mut self, fields: &'doc [Field<'doc, &'doc str>]) {
        for field in fields {
            self.validate_field(&field.name, field.position);
        }
    }

    fn validate_field(&mut self, name: &str, pos: Pos) {
        if is_snake_case(name) {
            self.errors.emit_error(pos, ErrorKind::FieldNameInSnakeCase);
        }
    }
}

pub struct UuidNameCaseValidator {
    pub errors: BTreeSet<Error>,
}

impl UuidNameCaseValidator {
    pub fn new() -> Self {
        Self {
            errors: Default::default(),
        }
    }
}

impl<'doc> SchemaVisitor<'doc> for UuidNameCaseValidator {
    fn visit_scalar_type(&mut self, scalar: &'doc ScalarType<'doc, &'doc str>) {
        if scalar.name == "UUID" {
            self.errors
                .emit_error(scalar.position, ErrorKind::UppercaseUuidScalar);
        }
    }
}

fn is_snake_case(s: &str) -> bool {
    s.contains('_') && s.to_snake_case() == s
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_is_snake_case() {
        assert!(is_snake_case("foo_bar"));
        assert!(is_snake_case("foo_bar_baz"));

        assert!(!is_snake_case("foo"));
        assert!(!is_snake_case("fooBar"));
        assert!(!is_snake_case("FooBar"));
    }
}
