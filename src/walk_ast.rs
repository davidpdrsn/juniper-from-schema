mod find_enum_variants;
mod find_interface_implementors;
mod find_special_scalar_types;

mod gen_juniper_code;
mod gen_query_trails;

pub use self::find_enum_variants::{find_enum_variants, EnumVariants};
pub use self::find_interface_implementors::{find_interface_implementors, InterfaceImplementors};
pub use self::find_special_scalar_types::{find_special_scalar_types, SpecialScalarTypesList};
// pub use self::gen_juniper_code::gen_juniper_code;
pub use self::gen_query_trails::gen_query_trails;

use graphql_parser::{
    query::Name,
    schema::{Document, Type},
};
use heck::CamelCase;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use std::iter::Extend;
use syn;
use syn::Ident;

pub struct Output {
    tokens: TokenStream,
    doc: Document,
    error_type: syn::Type,
    // special_scalars: SpecialScalarTypesList,
    // interface_implementors: InterfaceImplementors,
    // enum_variants: EnumVariants,
}

impl Output {
    pub fn new(doc: Document, error_type: syn::Type) -> Self {
        Output {
            tokens: quote! {},
            doc,
            error_type,
        }
    }

    // pub fn new(
    //     special_scalars: SpecialScalarTypesList,
    //     interface_implementors: InterfaceImplementors,
    //     enum_variants: EnumVariants,
    // ) -> Self {
    //     Output {
    //         tokens: quote! {},
    //         special_scalars,
    //         interface_implementors,
    //         enum_variants,
    //     }
    // }

    // pub fn tokens(self) -> TokenStream {
    //     self.tokens
    // }

    // fn is_date_time_scalar_defined(&self) -> bool {
    //     self.special_scalars.date_time_defined()
    // }

    // fn is_date_scalar_defined(&self) -> bool {
    //     self.special_scalars.date_defined()
    // }

    // fn is_scalar(&self, name: &str) -> bool {
    //     self.special_scalars.is_scalar(name)
    // }

    // fn interface_implementors(&self) -> &InterfaceImplementors {
    //     &self.interface_implementors
    // }

    // fn enum_variants(&self) -> &EnumVariants {
    //     &self.enum_variants
    // }

    // fn clone_without_tokens(&self) -> Self {
    //     Output {
    //         tokens: quote! {},
    //         special_scalars: self.special_scalars.clone(),
    //         interface_implementors: self.interface_implementors.clone(),
    //         enum_variants: self.enum_variants.clone(),
    //     }
    // }
}

impl Extend<TokenTree> for Output {
    fn extend<T: IntoIterator<Item = TokenTree>>(&mut self, iter: T) {
        self.tokens.extend(iter);
    }
}

impl Extend<TokenStream> for Output {
    fn extend<T: IntoIterator<Item = TokenStream>>(&mut self, iter: T) {
        self.tokens.extend(iter);
    }
}

pub fn ident<T: AsRef<str>>(name: T) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}

pub fn type_name(type_: &Type) -> Name {
    match *type_ {
        Type::NamedType(ref name) => name.clone(),
        Type::ListType(ref item_type) => type_name(&*item_type),
        Type::NonNullType(ref item_type) => type_name(&*item_type),
    }
}

// Type according to https://graphql.org/learn/schema/#scalar-types
pub fn graphql_scalar_type_to_rust_type(name: &str, out: &Output) -> (TokenStream, TypeKind) {
    unimplemented!()
    // match name {
    //     "Int" => (quote! { i32 }, TypeKind::Scalar),
    //     "Float" => (quote! { f64 }, TypeKind::Scalar),
    //     "String" => (quote! { String }, TypeKind::Scalar),
    //     "Boolean" => (quote! { bool }, TypeKind::Scalar),
    //     "ID" => (quote! { juniper::ID }, TypeKind::Scalar),
    //     "Date" => {
    //         if out.is_date_scalar_defined() {
    //             (quote! { chrono::naive::NaiveDate }, TypeKind::Scalar)
    //         } else {
    //             panic!(
    //                 "Fields with type `Date` is only allowed if you have defined a scalar named `Date`"
    //             )
    //         }
    //     }
    //     "DateTime" => {
    //         if out.is_date_time_scalar_defined() {
    //             (
    //                 quote! { chrono::DateTime<chrono::offset::Utc> },
    //                 TypeKind::Scalar,
    //             )
    //         } else {
    //             panic!(
    //                 "Fields with type `DateTime` is only allowed if you have defined a scalar named `DateTime`"
    //             )
    //         }
    //     }
    //     name => {
    //         if out.is_scalar(name) || out.enum_variants().contains(name) {
    //             (quote_ident(name.to_camel_case()), TypeKind::Scalar)
    //         } else {
    //             (quote_ident(name.to_camel_case()), TypeKind::Type)
    //         }
    //     }
    // }
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
