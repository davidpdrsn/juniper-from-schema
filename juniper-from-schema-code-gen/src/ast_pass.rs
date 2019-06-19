mod ast_data_pass;
mod code_gen_pass;
pub mod error;
mod schema_visitor;

pub use self::ast_data_pass::AstData;
pub use self::code_gen_pass::CodeGenPass;

use graphql_parser::{query::Name, schema::Type};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn;
use syn::Ident;

pub fn ident<T: AsRef<str>>(name: T) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}

pub fn type_name(type_: &Type) -> &Name {
    match *type_ {
        Type::NamedType(ref name) => &name,
        Type::ListType(ref item_type) => type_name(&*item_type),
        Type::NonNullType(ref item_type) => type_name(&*item_type),
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
