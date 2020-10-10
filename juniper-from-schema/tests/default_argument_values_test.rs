// #![allow(clippy::let_unit_value)]
// #![allow(dead_code, unused_variables, unused_imports)]

// #[macro_use]
// extern crate juniper;

// use assert_json_diff::assert_json_include;
// use juniper::{Executor, FieldResult, Variables};
// use juniper_from_schema::{graphql_schema, graphql_schema_from_file};
// use serde_json::{self, json, Value};
// use std::collections::HashMap;

// graphql_schema! {
//     type Query {
//         int(arg: Int = 1): Int! @juniper(ownership: "owned")

//         float(arg: Float = 1.5): Float! @juniper(ownership: "owned")

//         string(arg: String = "foo"): String! @juniper(ownership: "owned")

//         boolean(arg: Boolean = true): Boolean! @juniper(ownership: "owned")

//         list(arg: [Int!] = [1, 2, 3]): [Int!]! @juniper(ownership: "owned")

//         enumeration(arg: Unit = METER): UNIT! @juniper(ownership: "owned")

//         object(arg: CoordinateIn = { lat: 1.0, long: 2.0 }): CoordinateOut! @juniper(ownership: "owned")

//         objectNullable(arg: Pagination = { pageSize: null }): Int @juniper(ownership: "owned")

//         objectNullableSet(arg: Pagination = { pageSize: 1 }): Int @juniper(ownership: "owned")

//         objectNullablePartial(arg: A = { a: "a arg" }): [String]! @juniper(ownership: "owned")

//         objectNullableNesting(arg: B = { c: { x: 1 } }): [Int]! @juniper(ownership: "owned")
//     }

//     input CoordinateIn {
//         lat: Float!
//         long: Float!
//     }

//     type CoordinateOut {
//         lat: Float!
//         long: Float!
//     }

//     input Pagination {
//         pageSize: Int
//     }

//     input A {
//         a: String
//         b: String
//     }

//     input B {
//         c: C
//     }

//     input C {
//         x: Int
//     }

//     enum Unit { METER FOOT }

//     schema { query: Query }
// }

// pub struct Query;

// impl QueryFields for Query {
//     fn field_int(&self, _: &Executor<'_, Context>, arg: i32) -> FieldResult<i32> {
//         Ok(arg)
//     }

//     fn field_float(&self, _: &Executor<'_, Context>, arg: f64) -> FieldResult<f64> {
//         Ok(arg)
//     }

//     fn field_string(&self, _: &Executor<'_, Context>, arg: String) -> FieldResult<String> {
//         Ok(arg)
//     }

//     fn field_boolean(&self, _: &Executor<'_, Context>, arg: bool) -> FieldResult<bool> {
//         Ok(arg)
//     }

//     fn field_list(&self, _: &Executor<'_, Context>, arg: Vec<i32>) -> FieldResult<Vec<i32>> {
//         Ok(arg)
//     }

//     fn field_enumeration(
//         &self,
//         _: &Executor<'_, Context>,
//         _: &QueryTrail<'_, Unit, Walked>,
//         arg: Unit,
//     ) -> FieldResult<Unit> {
//         Ok(arg)
//     }

//     fn field_object(
//         &self,
//         _: &Executor<'_, Context>,
//         _: &QueryTrail<'_, CoordinateOut, Walked>,
//         arg: CoordinateIn,
//     ) -> FieldResult<CoordinateOut> {
//         Ok(CoordinateOut {
//             lat: arg.lat,
//             long: arg.long,
//         })
//     }

//     fn field_object_nullable(
//         &self,
//         _: &Executor<'_, Context>,
//         arg: Pagination,
//     ) -> FieldResult<Option<i32>> {
//         Ok(arg.page_size)
//     }

//     fn field_object_nullable_set(
//         &self,
//         _: &Executor<'_, Context>,
//         arg: Pagination,
//     ) -> FieldResult<Option<i32>> {
//         Ok(arg.page_size)
//     }

//     fn field_object_nullable_partial(
//         &self,
//         _: &Executor<'_, Context>,
//         arg: A,
//     ) -> FieldResult<Vec<Option<String>>> {
//         Ok(vec![arg.a, arg.b])
//     }

//     fn field_object_nullable_nesting(
//         &self,
//         _: &Executor<'_, Context>,
//         b: B,
//     ) -> FieldResult<Vec<Option<i32>>> {
//         Ok(vec![b.c.and_then(|c| c.x)])
//     }
// }

// pub struct CoordinateOut {
//     pub lat: f64,
//     pub long: f64,
// }

// impl CoordinateOutFields for CoordinateOut {
//     fn field_lat(&self, _: &Executor<'_, Context>) -> FieldResult<&f64> {
//         Ok(&self.lat)
//     }

//     fn field_long(&self, _: &Executor<'_, Context>) -> FieldResult<&f64> {
//         Ok(&self.long)
//     }
// }

// type Context = ();

// #[test]
// fn test_int() {
//     let value = run_query(r#"query { int }"#);
//     assert_json_include!(actual: value, expected: json!({ "int": 1 }));

//     let value = run_query(r#"query { int(arg: 1337) }"#);
//     assert_json_include!(actual: value, expected: json!({ "int": 1337 }));
// }

// #[test]
// fn test_float() {
//     let value = run_query(r#"query { float }"#);
//     assert_json_include!(actual: value, expected: json!({ "float": 1.5 }));

//     let value = run_query(r#"query { float(arg: 1337.5) }"#);
//     assert_json_include!(actual: value, expected: json!({ "float": 1337.5 }));
// }

// #[test]
// fn test_string() {
//     let value = run_query(r#"query { string }"#);
//     assert_json_include!(actual: value, expected: json!({ "string": "foo" }));

//     let value = run_query(r#"query { string(arg: "bar") }"#);
//     assert_json_include!(actual: value, expected: json!({ "string": "bar" }));
// }

// #[test]
// fn test_boolean() {
//     let value = run_query(r#"query { boolean }"#);
//     assert_json_include!(actual: value, expected: json!({ "boolean": true }));

//     let value = run_query(r#"query { boolean(arg: false) }"#);
//     assert_json_include!(actual: value, expected: json!({ "boolean": false }));
// }

// #[test]
// fn test_list() {
//     let value = run_query(r#"query { list }"#);
//     assert_json_include!(actual: value, expected: json!({ "list": [1, 2, 3] }));

//     let value = run_query(r#"query { list(arg: [1337]) }"#);
//     assert_json_include!(actual: value, expected: json!({ "list": [1337] }));
// }

// #[test]
// fn test_enumeration() {
//     let value = run_query(r#"query { enumeration }"#);
//     assert_json_include!(actual: value, expected: json!({ "enumeration": "METER" }));

//     let value = run_query(r#"query { enumeration(arg: FOOT) }"#);
//     assert_json_include!(actual: value, expected: json!({ "enumeration": "FOOT" }));
// }

// #[test]
// fn test_object() {
//     let value = run_query(r#"query { object { lat long } }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "object": { "lat": 1.0, "long": 2.0 } })
//     );

//     let value = run_query(r#"query { object(arg: { lat: 10.0, long: 20.0 }) { lat long } }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "object": { "lat": 10.0, "long": 20.0 } })
//     );
// }

// #[test]
// fn test_object_nullable() {
//     let value = run_query(r#"query { objectNullable }"#);
//     assert_json_include!(actual: value, expected: json!({ "objectNullable": null }));

//     let value = run_query(r#"query { objectNullable(arg: { pageSize: 1 }) }"#);
//     assert_json_include!(actual: value, expected: json!({ "objectNullable": 1 }));
// }

// #[test]
// fn test_object_nullable_set() {
//     let value = run_query(r#"query { objectNullableSet }"#);
//     assert_json_include!(actual: value, expected: json!({ "objectNullableSet": 1 }));

//     let value = run_query(r#"query { objectNullableSet(arg: { pageSize: 2 }) }"#);
//     assert_json_include!(actual: value, expected: json!({ "objectNullableSet": 2 }));

//     let value = run_query(r#"query { objectNullableSet(arg: { pageSize: null }) }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "objectNullableSet": null })
//     );
// }

// #[test]
// fn test_object_partial() {
//     let value = run_query(r#"query { objectNullablePartial }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "objectNullablePartial": ["a arg", null] })
//     );

//     let value = run_query(r#"query { objectNullablePartial(arg: { a: "a field" }) }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "objectNullablePartial": ["a field", null] })
//     );

//     let value = run_query(r#"query { objectNullablePartial(arg: { b: "b field" }) }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "objectNullablePartial": [null, "b field"] })
//     );

//     let value =
//         run_query(r#"query { objectNullablePartial(arg: { a: "a field", b: "b field" }) }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "objectNullablePartial": ["a field", "b field"] })
//     );
// }

// #[test]
// fn test_object_nesting() {
//     let value = run_query(r#"query { objectNullableNesting }"#);
//     assert_json_include!(
//         actual: value,
//         expected: json!({ "objectNullableNesting": [1] })
//     );
// }

// fn run_query(query: &str) -> Value {
//     let ctx = ();

//     let (res, _errors) = juniper::execute(
//         query,
//         None,
//         &Schema::new(Query, juniper::EmptyMutation::new()),
//         &Variables::new(),
//         &ctx,
//     )
//     .unwrap();

//     let json = serde_json::from_str(&serde_json::to_string(&res).unwrap()).unwrap();
//     println!("--- <json> -----------------");
//     println!("{}", serde_json::to_string_pretty(&json).unwrap());
//     println!("--- </json> -----------------");
//     json
// }
