#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../compile_pass/setup.rs");

juniper_from_schema::graphql_schema! {
    schema {
      query: Query
    }

    type Query {
      // The directive makes the return value `FieldResult<String>`
      // rather than the default `FieldResult<&String>`
      helloWorld(name: String!): String! @juniper(ownership: "owned")
    }

    scalar String
}

pub struct Query;

impl QueryFields for Query {
    fn field_hello_world(&self, executor: &Executor<Context>, name: String) -> FieldResult<String> {
        todo!()
    }
}
