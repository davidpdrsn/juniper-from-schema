use std::collections::BTreeMap;
use std::path::PathBuf;
use syn::{
    self, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token, Type,
};

struct IdentMap(BTreeMap<String, bool>);

impl Parse for IdentMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let b = bracketed!(content in input);
        let punc = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
        let mut map = BTreeMap::<String, bool>::new();
        for ident in punc.into_iter() {
            let key = ident.to_string();
            if map.contains_key(&key) {
                return Err(syn::parse::Error::new(
                    b.span,
                    format!("argument `with_idents` has duplicated element `{}`", key),
                ));
            } else {
                map.insert(key, false);
            }
        }
        if map.len() == 0 {
            Err(syn::parse::Error::new(
                b.span,
                "argument `with_idents` should have at least one element",
            ))
        } else {
            Ok(IdentMap(map))
        }
    }
}

#[derive(Debug)]
pub struct GraphqlSchemaFromFileInput {
    pub schema_path: PathBuf,
    pub error_type: Type,
    pub with_idents: Option<BTreeMap<String, bool>>,
}

impl Parse for GraphqlSchemaFromFileInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file = input.parse::<syn::LitStr>()?.value();
        let cargo_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Env var `CARGO_MANIFEST_DIR` was missing");
        let pwd = PathBuf::from(cargo_dir);
        let schema_path = pwd.join(file);

        let mut error_type: Option<Type> = None;
        let mut with_idents: Option<IdentMap> = None;

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if !input.is_empty() {
                let ident = input.parse::<syn::Ident>()?;
                match ident.to_string().as_ref() {
                    "error_type" => {
                        parse_optional_arg(&mut error_type, &input, &ident)?;
                    }
                    "with_idents" => {
                        parse_optional_arg(&mut with_idents, &input, &ident)?;
                    }
                    _ => {
                        return Err(syn::parse::Error::new(
                            ident.span(),
                            "expected `error_type` or `with_idents`",
                        ));
                    }
                }
            }
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        let error_type = error_type.unwrap_or_else(|| default_error_type());
        let with_idents = with_idents.map(|t| t.0);
        Ok(GraphqlSchemaFromFileInput {
            schema_path,
            error_type,
            with_idents,
        })
    }
}

pub fn default_error_type() -> Type {
    syn::parse_str("juniper::FieldError").expect("Failed to parse default error type")
}

fn parse_optional_arg<T: Parse>(
    value: &mut Option<T>,
    input: &ParseStream,
    ident: &Ident,
) -> syn::Result<()> {
    input.parse::<Token![:]>()?;
    if value.is_none() {
        *value = Some(input.parse::<T>()?);
        Ok(())
    } else {
        Err(syn::parse::Error::new(
            ident.span(),
            "duplicate argument definition",
        ))
    }
}
