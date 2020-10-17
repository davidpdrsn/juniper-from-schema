pub mod ast_data_pass;
pub mod code_gen_pass;
pub mod directive_parsing;
pub mod error;
pub mod schema_visitor;
pub mod validations;

pub use self::code_gen_pass::CodeGenPass;
pub use self::error::ErrorKind;

use self::error::Error;
use graphql_parser::schema::Type;
use graphql_parser::Pos;
use std::collections::BTreeSet;

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

pub trait EmitError<'doc> {
    fn emit_error(&mut self, pos: Pos, kind: ErrorKind<'doc>);
}

impl<'doc> EmitError<'doc> for BTreeSet<Error<'doc>> {
    fn emit_error(&mut self, pos: Pos, kind: ErrorKind<'doc>) {
        let error = Error { pos, kind };
        self.insert(error);
    }
}
