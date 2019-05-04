use regex::Regex;
use std::path::PathBuf;
use syn::Type;

#[derive(Debug)]
pub struct ParseCodeGenPass {
    pub schema_path: PathBuf,
    pub error_type: Type,
}

pub fn parse_input(input: &str) -> Result<ParseCodeGenPass, ()> {
    let re_without_error_type = Regex::new(r#"^"(?P<file>[^"]+)"$"#).expect("invalid regex");

    let re_with_error_type =
        Regex::new(r#"^"(?P<file>[^"]+)" *, *error_type *: *(?P<error_type>[^,]+),?$"#)
            .expect("invalid regex");

    let file;
    let error_type: Type;

    match re_with_error_type.captures(input) {
        Some(caps) => {
            file = (&caps["file"]).to_string();

            let error_type_name = &caps["error_type"];
            error_type = syn::parse_str(error_type_name)
                .expect("Failed to parse error_type from input into name of a Rust type");
        }
        None => match re_without_error_type.captures(input) {
            Some(caps) => {
                file = (&caps["file"]).to_string();
                error_type = default_error_type();
            }
            None => {
                let mut msg = "Invalid input given to `graphql_schema_from_file`".to_string();
                msg.push_str("\n");
                msg.push_str("Valid inputs look like:");
                msg.push_str("\n");
                msg.push_str(r#"  graphql_schema_from_file!("path_to_schema.graphql")"#);
                msg.push_str("\n");
                msg.push_str(r#"  graphql_schema_from_file!("path_to_schema.graphql", error_type: YourErrorType)"#);
                panic!("{}", msg)
            }
        },
    };

    let cargo_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Env var `CARGO_MANIFEST_DIR` was missing");
    let pwd = PathBuf::from(cargo_dir);
    let schema_path = pwd.join(file);

    Ok(ParseCodeGenPass {
        schema_path,
        error_type,
    })
}

pub fn default_error_type() -> Type {
    syn::parse_str("juniper::FieldError").expect("Failed to parse default error type")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_input_without_error_type() {
        let path = "tests/schemas/complex_schema.graphql";
        let output = parse_input(&format!("\"{}\"", path)).unwrap();

        assert!(output.schema_path.to_str().unwrap().contains(path));
    }

    #[test]
    fn parsing_input_with_error_type() {
        let path = "tests/schemas/complex_schema.graphql";
        let input = format!("\"{}\", error_type: MyError", path);
        let output = parse_input(&input).unwrap();

        let type_ = syn::parse_str("MyError").unwrap();
        assert_eq!(output.error_type, type_);
    }

    #[test]
    fn parsing_input_from_token_stream() {
        use quote::quote;
        let stream = quote! { "foo.graphql", error_type: foo::bar::MyError };
        let input = stream.to_string();

        let output = parse_input(&input).unwrap();

        let type_ = syn::parse_str("foo::bar::MyError").unwrap();
        assert_eq!(output.error_type, type_);

        assert!(output.schema_path.to_str().unwrap().contains("foo.graphql"));
    }

    #[test]
    fn allows_trailing_commas() {
        use quote::quote;
        let stream = quote! { "foo.graphql", error_type: foo::bar::MyError, };
        let input = stream.to_string();

        let output = parse_input(&input).unwrap();

        let type_ = syn::parse_str("foo::bar::MyError").unwrap();
        assert_eq!(output.error_type, type_);

        assert!(output.schema_path.to_str().unwrap().contains("foo.graphql"));
    }
}
