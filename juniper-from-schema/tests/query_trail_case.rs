#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate maplit;

use assert_json_diff::assert_json_include;
use juniper::{EmptyMutation, Executor, FieldResult, Variables, ID};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};
use serde_json::{self, json, Value};
use std::collections::HashMap;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        field: Field! @juniper(ownership: "owned")
    }

    type Field {
        some_nested_queried: Boolean!
        some_nested: SomeNested! @juniper(ownership: "owned")
    }

    type SomeNested {
        query_here_with_camel_case_fields: String! @juniper(ownership: "owned")
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_field(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Field, Walked>,
    ) -> FieldResult<Field> {
        let some_nested_queried = trail.some_nested().walk().is_some()
            && trail.some_nested().query_here_with_camel_case_fields();

        Ok(Field {
            some_nested_queried,
        })
    }
}

pub struct Field {
    some_nested_queried: bool,
}

impl FieldFields for Field {
    fn field_some_nested_queried(&self, executor: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&self.some_nested_queried)
    }

    fn field_some_nested(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, SomeNested, Walked>,
    ) -> FieldResult<SomeNested> {
        Ok(SomeNested)
    }
}

pub struct SomeNested;

impl SomeNestedFields for SomeNested {
    fn field_query_here_with_camel_case_fields(
        &self,
        executor: &Executor<'_, Context>,
    ) -> FieldResult<String> {
        Ok("OK".to_string())
    }
}

pub struct Context;

impl juniper::Context for Context {}

#[test]
fn correct_casing_on_query_trails() {
    let value = run_query(
        r#"
        query Foo {
            field {
                someNestedQueried
                someNested {
                    queryHereWithCamelCaseFields
                }
            }
        }
        "#,
    );

    assert_json_include!(
        actual: value,
        expected: json!({
            "field": { "someNestedQueried": true },
        })
    );
}

fn run_query(query: &str) -> Value {
    let ctx = Context;

    let (res, _errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    serde_json::from_str(&serde_json::to_string(&res).unwrap()).unwrap()
}
