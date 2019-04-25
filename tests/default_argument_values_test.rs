#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate juniper;

use assert_json_diff::assert_json_include;
use juniper::{Executor, FieldResult, Variables};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};
use serde_json::{self, json, Value};
use std::collections::HashMap;

graphql_schema! {
    type Query {
        "#[ownership(owned)]"
        int(arg: Int = 1): Int!

        "#[ownership(owned)]"
        float(arg: Float = 1.5): Float!

        "#[ownership(owned)]"
        string(arg: String = "foo"): String!

        "#[ownership(owned)]"
        boolean(arg: Boolean = true): Boolean!

        "#[ownership(owned)]"
        list(arg: [Int!] = [1, 2, 3]): [Int!]!

        "#[ownership(owned)]"
        enumeration(arg: Unit = METER): UNIT!
    }

    enum Unit { METER FOOT }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_int(&self, _: &Executor<'_, Context>, arg: i32) -> FieldResult<i32> {
        Ok(arg)
    }

    fn field_float(&self, _: &Executor<'_, Context>, arg: f64) -> FieldResult<f64> {
        Ok(arg)
    }

    fn field_string(&self, _: &Executor<'_, Context>, arg: String) -> FieldResult<String> {
        Ok(arg)
    }

    fn field_boolean(&self, _: &Executor<'_, Context>, arg: bool) -> FieldResult<bool> {
        Ok(arg)
    }

    fn field_list(&self, _: &Executor<'_, Context>, arg: Vec<i32>) -> FieldResult<Vec<i32>> {
        Ok(arg)
    }

    fn field_enumeration(
        &self,
        _: &Executor<'_, Context>,
        _: &QueryTrail<'_, Unit, Walked>,
        arg: Unit,
    ) -> FieldResult<Unit> {
        Ok(arg)
    }
}

type Context = ();

#[test]
fn test_int() {
    let value = run_query(r#"query { int }"#);
    assert_json_include!(actual: value, expected: json!({ "int": 1 }));

    let value = run_query(r#"query { int(arg: 1337) }"#);
    assert_json_include!(actual: value, expected: json!({ "int": 1337 }));
}

#[test]
fn test_float() {
    let value = run_query(r#"query { float }"#);
    assert_json_include!(actual: value, expected: json!({ "float": 1.5 }));

    let value = run_query(r#"query { float(arg: 1337.5) }"#);
    assert_json_include!(actual: value, expected: json!({ "float": 1337.5 }));
}

#[test]
fn test_string() {
    let value = run_query(r#"query { string }"#);
    assert_json_include!(actual: value, expected: json!({ "string": "foo" }));

    let value = run_query(r#"query { string(arg: "bar") }"#);
    assert_json_include!(actual: value, expected: json!({ "string": "bar" }));
}

#[test]
fn test_boolean() {
    let value = run_query(r#"query { boolean }"#);
    assert_json_include!(actual: value, expected: json!({ "boolean": true }));

    let value = run_query(r#"query { boolean(arg: false) }"#);
    assert_json_include!(actual: value, expected: json!({ "boolean": false }));
}

#[test]
fn test_list() {
    let value = run_query(r#"query { list }"#);
    assert_json_include!(actual: value, expected: json!({ "list": [1, 2, 3] }));

    let value = run_query(r#"query { list(arg: [1337]) }"#);
    assert_json_include!(actual: value, expected: json!({ "list": [1337] }));
}

#[test]
fn test_enumeration() {
    let value = run_query(r#"query { enumeration }"#);
    assert_json_include!(actual: value, expected: json!({ "enumeration": "METER" }));

    let value = run_query(r#"query { enumeration(arg: FOOT) }"#);
    assert_json_include!(actual: value, expected: json!({ "enumeration": "FOOT" }));
}

fn run_query(query: &str) -> Value {
    let ctx = ();

    let (res, _errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, juniper::EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    let json = serde_json::from_str(&serde_json::to_string(&res).unwrap()).unwrap();
    println!("--- <json> -----------------");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
    println!("--- </json> -----------------");
    json
}
