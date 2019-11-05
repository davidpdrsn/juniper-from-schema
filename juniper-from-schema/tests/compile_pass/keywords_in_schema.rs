#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

use juniper::{EmptyMutation, Executor, FieldResult, Variables};
use serde_json::Value;

pub struct Context;

impl juniper::Context for Context {}

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        type(fn: Input!): [String!]! @juniper(ownership: "owned")
    }

    input Input {
        struct: String!
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_type(&self, _: &Executor<'_, Context>, input: Input) -> FieldResult<Vec<String>> {
        Ok(vec![input.r#struct])
    }
}

fn main() {
    let (juniper_value, _errors) = juniper::execute(
        r#"
        query Test {
            type(fn: { struct: "value" })
        }"#,
        None,
        &Schema::new(Query, juniper::EmptyMutation::new()),
        &Variables::new(),
        &Context,
    )
    .unwrap_or_else(|e| {
        let json = serde_json::to_string_pretty(&e).unwrap();
        panic!("{}", json)
    });

    let json: Value =
        serde_json::from_str(&serde_json::to_string(&juniper_value).unwrap()).unwrap();

    assert_json_diff::assert_json_include!(
        actual: json,
        expected: serde_json::json!({ "type": ["value"] })
    );
}
