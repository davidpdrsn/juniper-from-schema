use std::path::PathBuf;
use syn::{
    self,
    parse::{Parse, ParseStream},
    Token, Type,
};

#[derive(Debug)]
pub struct GraphqlSchemaFromFileInput {
    pub schema_path: PathBuf,
    pub error_type: Type,
}

impl Parse for GraphqlSchemaFromFileInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file = input.parse::<syn::LitStr>()?.value();
        let cargo_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Env var `CARGO_MANIFEST_DIR` was missing");
        let pwd = PathBuf::from(cargo_dir);
        let schema_path = pwd.join(file);

        let error_type = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let ident = input.parse::<syn::Ident>()?;
            if ident != "error_type" {
                return Err(syn::parse::Error::new(
                    ident.span(),
                    "expected `error_type`",
                ));
            }
            input.parse::<Token![:]>()?;
            input.parse::<Type>()?
        } else {
            default_error_type()
        };

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(GraphqlSchemaFromFileInput {
            schema_path,
            error_type,
        })
    }
}

pub fn default_error_type() -> Type {
    syn::parse_str("juniper::FieldError").expect("Failed to parse default error type")
}
