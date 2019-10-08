#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate juniper;

use assert_json_diff::assert_json_include;
use juniper::{Executor, FieldResult, Variables};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};
use serde_json::{self, json, Value};
use std::collections::HashMap;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        a: A! @juniper(ownership: "owned")
    }

    type A {
        b: B! @juniper(ownership: "owned")
    }

    type B {
        c: C! @juniper(ownership: "owned")
    }

    type C {
        fieldWithArg(
            stringArg: String!
            nullableArg: String
            nullableArg2: String
            intArg: Int!
            floatArg: Float!
            boolArg: Boolean!
            listArg: [Int!]!
            enumArg: Color!
        ): String! @juniper(ownership: "owned")
    }

    enum Color {
        RED
        BLUE
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_a(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, A, Walked>,
    ) -> FieldResult<A> {
        assert_eq!(
            Some("foo".to_string()),
            trail.b().c().field_with_arg_args().string_arg()
        );
        assert_eq!(
            Some(None),
            trail.b().c().field_with_arg_args().nullable_arg()
        );
        assert_eq!(
            Some(Some("bar".to_string())),
            trail.b().c().field_with_arg_args().nullable_arg2()
        );
        assert_eq!(Some(1), trail.b().c().field_with_arg_args().int_arg());
        assert_eq!(Some(2.5), trail.b().c().field_with_arg_args().float_arg());
        assert_eq!(Some(false), trail.b().c().field_with_arg_args().bool_arg());
        assert_eq!(
            Some(vec![1, 2, 3]),
            trail.b().c().field_with_arg_args().list_arg()
        );
        assert_eq!(
            Some(Color::Red),
            trail.b().c().field_with_arg_args().enum_arg()
        );

        Ok(A)
    }
}

// TODO: Support default arguments

pub struct A;

impl AFields for A {
    fn field_b(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, B, Walked>,
    ) -> FieldResult<B> {
        Ok(B)
    }
}

pub struct B;

impl BFields for B {
    fn field_c(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, C, Walked>,
    ) -> FieldResult<C> {
        Ok(C)
    }
}

pub struct C;

impl CFields for C {
    fn field_field_with_arg(
        &self,
        executor: &Executor<'_, Context>,
        _: String,
        _: Option<String>,
        _: Option<String>,
        _: i32,
        _: f64,
        _: bool,
        _: Vec<i32>,
        _: Color,
    ) -> FieldResult<String> {
        Ok(String::new())
    }
}

#[test]
fn scalar_values() {
    let value = run_query(
        r#"query {
        a {
            b {
                c {
                    fieldWithArg(
                        stringArg: "foo",
                        nullableArg: null,
                        nullableArg2: "bar",
                        intArg: 1,
                        floatArg: 2.5,
                        boolArg: false,
                        listArg: [1, 2, 3],
                        enumArg: RED
                    )
                }
            }
        }
    }"#,
    );
    assert_json_include!(
        actual: value,
        expected: json!({
            "a": { "b": { "c": {} } }
        })
    );
}

type Context = ();

fn run_query(query: &str) -> Value {
    let (res, _errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, juniper::EmptyMutation::new()),
        &Variables::new(),
        &(),
    )
    .unwrap();

    let json = serde_json::from_str(&serde_json::to_string(&res).unwrap()).unwrap();
    println!("--- <json> -----------------");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
    println!("--- </json> -----------------");
    json
}
