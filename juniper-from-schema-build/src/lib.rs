//! Use juniper-from-schema from build.rs
//!
//! # Example
//!
//! ## Required dependencies
//!
//! ```text
//! [dependencies]
//! juniper-from-schema = <juniper-from-schema-version>
//!
//! [build-dependencies]
//! juniper-from-schema-build = <juniper-from-schema-version>
//! ```
//!
//! ## `build.rs`
//!
//! ```no_run
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     juniper_from_schema_build::compile_schema_literal(r#"
//!         schema {
//!             query: Query
//!         }
//!
//!         type Query {
//!             ping: Boolean!
//!         }
//!     "#)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## `main.rs` or `lib.rs`
//!
//! ```ignore
//! juniper_from_schema::include_schema!();
//!
//! // the rest of your code...
//! ```

#![deny(
    dead_code,
    missing_docs,
    mutable_borrow_reservation_conflict,
    unused_imports,
    unused_must_use,
    unused_variables
)]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/juniper-from-schema-build/0.5.2")]

use std::error::Error;
use std::fs;
use std::path::Path;
use std::{env, path::PathBuf};

/// Simple compilation of a GraphQL schema literal.
pub fn compile_schema_literal(schema: &str) -> Result<(), Box<dyn Error>> {
    configure_for_schema_literal(schema).compile()
}

/// Configure a [`CodeGen`] with a GraphQL schema literal.
///
/// [`CodeGen`]: struct.CodeGen.html
pub fn configure_for_schema_literal(schema: &str) -> CodeGen {
    CodeGen {
        schema: SchemaLocation::Literal(schema.to_string()),
        context_type: None,
        error_type: None,
    }
}

/// Simple compilation of a GraphQL schema file.
pub fn compile_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    configure_for_file(path).compile()
}

/// Configure a [`CodeGen`] with a GraphQL schema file.
///
/// [`CodeGen`]: struct.CodeGen.html
pub fn configure_for_file<P: AsRef<Path>>(path: P) -> CodeGen {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let path = root.join(path);

    CodeGen {
        schema: SchemaLocation::File(path),
        context_type: None,
        error_type: None,
    }
}

/// GraphQL schema compiler.
#[derive(Debug)]
pub struct CodeGen {
    schema: SchemaLocation,
    context_type: Option<Result<syn::Type, Box<dyn Error>>>,
    error_type: Option<Result<syn::Type, Box<dyn Error>>>,
}

#[derive(Debug)]
enum SchemaLocation {
    File(PathBuf),
    Literal(String),
}

impl CodeGen {
    /// Set the context type you want to use.
    ///
    /// Will be parsed to a Rust type using [`syn::parse_str`].
    ///
    /// [`syn::parse_str`]: https://docs.rs/syn/1.0.48/syn/fn.parse_str.html
    pub fn context_type(mut self, context_type: &str) -> Self {
        self.context_type = Some(syn::parse_str(context_type).map_err(From::from));
        self
    }

    /// Set the error type you want to use.
    ///
    /// Will be parsed to a Rust type using [`syn::parse_str`].
    ///
    /// [`syn::parse_str`]: https://docs.rs/syn/1.0.48/syn/fn.parse_str.html
    pub fn error_type(mut self, error_type: &str) -> Self {
        self.error_type = Some(syn::parse_str(error_type).map_err(From::from));
        self
    }

    /// Compile the GraphQL schema.
    pub fn compile(self) -> Result<(), Box<dyn Error>> {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("juniper_from_schema_graphql_schema.rs");

        let mut code_gen = match self.schema {
            SchemaLocation::File(path) => {
                juniper_from_schema_code_gen::CodeGen::build_from_schema_file(path)
            }
            SchemaLocation::Literal(schema) => {
                juniper_from_schema_code_gen::CodeGen::build_from_schema_literal(schema)
            }
        };

        if let Some(context_type) = self.context_type {
            code_gen = code_gen.context_type(context_type?);
        }

        if let Some(error_type) = self.error_type {
            code_gen = code_gen.error_type(error_type?);
        }

        let code = code_gen.finish().generate_code()?;

        fs::write(&dest_path, code.to_string())?;

        println!("cargo:rerun-if-changed=build.rs");

        Ok(())
    }
}
