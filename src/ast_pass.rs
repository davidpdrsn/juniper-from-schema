mod ast_data_pass;
mod code_gen_pass;

pub use self::ast_data_pass::AstData;
pub use self::code_gen_pass::CodeGenPass;

use graphql_parser::{query::Name, schema::Type};
use heck::CamelCase;
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

// Type according to https://graphql.org/learn/schema/#scalar-types
pub fn graphql_scalar_type_to_rust_type(name: &str, out: &CodeGenPass) -> (TokenStream, TypeKind) {
    match name {
        "Int" => (quote! { i32 }, TypeKind::Scalar),
        "Float" => (quote! { f64 }, TypeKind::Scalar),
        "String" => (quote! { String }, TypeKind::Scalar),
        "Boolean" => (quote! { bool }, TypeKind::Scalar),
        "ID" => (quote! { juniper::ID }, TypeKind::Scalar),
        "Date" => {
            if out.is_date_scalar_defined() {
                (quote! { chrono::naive::NaiveDate }, TypeKind::Scalar)
            } else {
                panic!(
                    "Fields with type `Date` is only allowed if you have defined a scalar named `Date`"
                )
            }
        }
        "DateTime" => {
            if out.is_date_time_scalar_defined() {
                (
                    quote! { chrono::DateTime<chrono::offset::Utc> },
                    TypeKind::Scalar,
                )
            } else {
                panic!(
                    "Fields with type `DateTime` is only allowed if you have defined a scalar named `DateTime`"
                )
            }
        }
        name => {
            if out.is_scalar(name) || out.is_enum(name) {
                (quote_ident(name.to_camel_case()), TypeKind::Scalar)
            } else {
                (quote_ident(name.to_camel_case()), TypeKind::Type)
            }
        }
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
