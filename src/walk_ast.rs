mod find_special_scalar_types;
mod gen_juniper_code;
mod gen_query_trails;

pub use self::find_special_scalar_types::{find_special_scalar_types, SpecialScalarTypesList};
pub use self::gen_juniper_code::gen_juniper_code;
pub use self::gen_query_trails::gen_query_trails;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::Ident;

pub struct Output {
    tokens: Vec<TokenStream>,
    special_scalars: SpecialScalarTypesList,
}

impl Output {
    pub fn new(special_scalars: SpecialScalarTypesList) -> Self {
        Output {
            tokens: vec![],
            special_scalars,
        }
    }

    pub fn tokens(self) -> Vec<TokenStream> {
        self.tokens
    }

    fn push(&mut self, toks: TokenStream) {
        self.tokens.push(toks);
    }

    fn is_date_time_scalar_defined(&self) -> bool {
        self.special_scalars.date_defined()
    }

    fn is_date_scalar_defined(&self) -> bool {
        self.special_scalars.date_time_defined()
    }

    fn clone_without_tokens(&self) -> Self {
        Output {
            tokens: vec![],
            special_scalars: self.special_scalars.clone(),
        }
    }
}

pub trait AddToOutput {
    fn add_to(self, out: &mut Output);
}

impl AddToOutput for TokenStream {
    fn add_to(self, out: &mut Output) {
        out.push(self);
    }
}

pub fn ident<T: AsRef<str>>(name: T) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}
