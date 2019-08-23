#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

use juniper::ID;

graphql_schema_from_file!(
    "../../../juniper-from-schema/tests/schemas/complex_schema.graphql",
    error_type: MyError,
    with_idents: [WrongIdent]
);