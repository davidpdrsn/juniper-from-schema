use std::{fmt::Write, path::PathBuf};
use syn::{
    self,
    parse::{Parse, ParseStream},
    Ident, Token, Type,
};

#[derive(Debug)]
pub struct GraphqlSchemaFromFileInput {
    pub schema_paths: Vec<PathBuf>,
    pub error_type: Option<Type>,
    pub context_type: Option<Type>,
}

impl Parse for GraphqlSchemaFromFileInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file = input.parse::<syn::LitStr>()?.value();
        let cargo_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Env var `CARGO_MANIFEST_DIR` was missing");
        let pwd = PathBuf::from(cargo_dir);
        let schema_path = pwd.join(file);

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let mut error_type = None::<Type>;
        let mut context_type = None::<Type>;

        loop {
            if input.is_empty() {
                break;
            }

            let key = input.parse::<Ident>()?;
            match &*key.to_string() {
                "error_type" => {
                    input.parse::<Token![:]>()?;
                    error_type = Some(input.parse()?);
                }
                "context_type" => {
                    input.parse::<Token![:]>()?;
                    context_type = Some(input.parse()?);
                }
                other => {
                    let mut msg = String::new();
                    writeln!(msg, "Unknown `graphql_schema_from_file` config `{}`", other).unwrap();
                    writeln!(msg, "Supported configs are `error_type` and `context_type`").unwrap();
                    return Err(syn::parse::Error::new(key.span(), msg));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(GraphqlSchemaFromFileInput {
            // Only supporting single paths for macro, since long term we're
            // moving towards the build.rs style
            schema_paths: vec![schema_path],
            error_type,
            context_type,
        })
    }
}
