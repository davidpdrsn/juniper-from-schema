pub mod ast_data_pass;
pub mod code_gen_pass;
pub mod directive_parsing;
pub mod error;
mod is_keyword;
pub mod schema_visitor;

pub use self::{code_gen_pass::CodeGenPass, error::ErrorKind};

use graphql_parser::{query::Name, schema::Type, Pos};
use is_keyword::is_keyword;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{self, Ident};

pub fn ident<T: AsRef<str>>(name: T) -> Ident {
    let r = name.as_ref();

    if is_keyword(r) {
        format_ident!("r#{}", r)
    } else {
        format_ident!("{}", r)
    }
}

// TODO: `Name` is a type alias for `String`. It would be nice if this returned a newtype so we
// would have more type safety. All this `&str` business makes me nervous.
pub fn type_name(type_: &Type) -> &Name {
    match &*type_ {
        Type::NamedType(name) => &name,
        Type::ListType(item_type) => type_name(&*item_type),
        Type::NonNullType(item_type) => type_name(&*item_type),
    }
}

pub fn quote_ident<T: AsRef<str>>(name: T) -> TokenStream {
    let ident = ident(name);
    quote! { #ident }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
    Scalar,
    Type,
}

pub trait EmitError<'doc> {
    fn emit_non_fatal_error(&mut self, pos: Pos, kind: ErrorKind<'doc>);
}
