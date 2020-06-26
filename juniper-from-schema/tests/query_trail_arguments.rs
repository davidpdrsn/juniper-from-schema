#![allow(clippy::too_many_arguments)]
#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate juniper;

use assert_json_diff::assert_json_include;
use chrono::prelude::*;
use juniper::{Executor, FieldResult, Variables, ID};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};
use serde_json::{self, json, Value};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

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
            objectArg: InputObject!
            cursorArg: Cursor!
            idArg: ID!
            urlArg: Url!
            uuidArg: Uuid!
            dateArg: Date!
            dateTimeArg: DateTimeUtc!
            defaultArg: String = "value set in schema"
            defaultArg2: String = "error"
        ): String! @juniper(ownership: "owned")

        fieldWithArgReturningType(
            stringArg: String!
        ): D! @juniper(ownership: "owned")
    }

    type D {
      value: String! @juniper(ownership: "owned")
    }

    input InputObject {
        value: String!
    }

    enum Color {
        RED
        BLUE
    }

    scalar Cursor
    scalar Url
    scalar Uuid
    scalar Date
    scalar DateTimeUtc
}

pub struct Query(bool);

impl QueryFields for Query {
    fn field_a(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, A, Walked>,
    ) -> FieldResult<A> {
        if let Some(c) = trail.b().c().walk() {
            if opt_check {
                assert_eq!(None, c.field_with_arg_args().nullable_arg2());
            } else {
                assert_eq!(
                    Some("bar".to_string()),
                    c.field_with_arg_args().nullable_arg2()
                );
            }
            assert_eq!("foo".to_string(), c.field_with_arg_args().string_arg());
            assert_eq!(None, c.field_with_arg_args().nullable_arg());
            assert_eq!(1, c.field_with_arg_args().int_arg());
            assert_eq!("2.5", c.field_with_arg_args().float_arg().to_string());
            assert_eq!(false, c.field_with_arg_args().bool_arg());
            assert_eq!(vec![1, 2, 3], c.field_with_arg_args().list_arg());
            assert_eq!(Color::Red, c.field_with_arg_args().enum_arg());
            assert_eq!(
                "baz".to_string(),
                c.field_with_arg_args().object_arg().value
            );
            assert_eq!(
                Cursor("cursor-value".to_string()),
                c.field_with_arg_args().cursor_arg()
            );
            assert_eq!(ID::new("id-value"), c.field_with_arg_args().id_arg());
            assert_eq!(
                Url::parse("https://example.net").unwrap(),
                c.field_with_arg_args().url_arg()
            );
            assert_eq!(
                Uuid::parse_str("46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e").unwrap(),
                c.field_with_arg_args().uuid_arg()
            );
            assert_eq!(
                NaiveDate::parse_from_str("2019-01-01", "%Y-%m-%d").unwrap(),
                c.field_with_arg_args().date_arg()
            );
            assert_eq!(
                DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap(),
                c.field_with_arg_args().date_time_arg()
            );
            assert_eq!(
                "value set in schema".to_string(),
                c.field_with_arg_args().default_arg()
            );
            assert_eq!(
                "value set in query".to_string(),
                c.field_with_arg_args().default_arg2()
            );
            assert_eq!(
                "qux".to_string(),
                c.field_with_arg_returning_type_args().string_arg()
            );
        }

        Ok(A)
    }
}

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
        _: InputObject,
        _: Cursor,
        _: ID,
        _: Url,
        _: Uuid,
        _: NaiveDate,
        _: DateTime<Utc>,
        _: String,
        _: String,
    ) -> FieldResult<String> {
        Ok(String::new())
    }

    fn field_field_with_arg_returning_type(
        &self,
        executor: &Executor<'_, Context>,
        _: &QueryTrail<'_, D, Walked>,
        _: String,
    ) -> FieldResult<D> {
        Ok(D)
    }
}

pub struct D;

impl DFields for D {
    fn field_value(&self, executor: &Executor<'_, Context>) -> FieldResult<String> {
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
                        enumArg: RED,
                        objectArg: { value: "baz" },
                        cursorArg: "cursor-value",
                        idArg: "id-value",
                        urlArg: "https://example.net",
                        uuidArg: "46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e",
                        dateArg: "2019-01-01",
                        dateTimeArg: "1996-12-19T16:39:57-08:00",
                        defaultArg2: "value set in query",
                    )
                    fieldWithArgReturningType(
                        stringArg: "qux",
                    ) {
                      value
                    }
                }
            }
        }
    }"#, false,
    );
    assert_json_include!(
        actual: value,
        expected: json!({
            "a": { "b": { "c": {} } }
        })
    );
}

#[test]
fn scalar_values_opt() {
    let value = run_query(
        r#"query {
        a {
            b {
                c {
                    fieldWithArg(
                        stringArg: "foo",
                        nullableArg: null,
                        intArg: 1,
                        floatArg: 2.5,
                        boolArg: false,
                        listArg: [1, 2, 3],
                        enumArg: RED,
                        objectArg: { value: "baz" },
                        cursorArg: "cursor-value",
                        idArg: "id-value",
                        urlArg: "https://example.net",
                        uuidArg: "46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e",
                        dateArg: "2019-01-01",
                        dateTimeArg: "1996-12-19T16:39:57-08:00",
                        defaultArg2: "value set in query",
                    )
                    fieldWithArgReturningType(
                        stringArg: "qux",
                    ) {
                      value
                    }
                }
            }
        }
    }"#, true,
    );
    assert_json_include!(
        actual: value,
        expected: json!({
            "a": { "b": { "c": {} } }
        })
    );
}

type Context = ();

fn run_query(query: &str, opt_check: bool) -> Value {
    let (res, _errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query(opt_check), juniper::EmptyMutation::new()),
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
