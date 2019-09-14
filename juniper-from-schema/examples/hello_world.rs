#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate juniper;

use juniper::*;
use juniper_from_schema::graphql_schema;

// This is the important line
graphql_schema! {
    schema {
      query: Query
      mutation: Mutation
    }

    type Query {
      // The directive makes the return value `FieldResult<String>`
      // rather than the default `FieldResult<&String>`
      helloWorld(name: String!): String! @juniper(ownership: "owned")
    }

    type Mutation {
      noop: Boolean!
    }
}

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

impl QueryFields for Query {
    fn field_hello_world(
        &self,
        executor: &Executor<'_, Context>,
        name: String,
    ) -> FieldResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_noop(&self, executor: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&true)
    }
}

fn main() {
    let ctx = Context;

    let query = "query { helloWorld(name: \"Ferris\") }";

    let (result, errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, Mutation),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    assert_eq!(errors.len(), 0);
    assert_eq!(
        result
            .as_object_value()
            .unwrap()
            .get_field_value("helloWorld")
            .unwrap()
            .as_scalar_value::<String>()
            .unwrap(),
        "Hello, Ferris!",
    );
}
