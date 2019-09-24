#![recursion_limit = "128"]
#![allow(dead_code)]
#![deny(deprecated)]

extern crate juniper;
extern crate maplit;

use assert_json_diff::assert_json_include;
use juniper::{Executor, FieldResult, Variables, ID};
use juniper_from_schema::graphql_schema_from_file;
use serde_json::{self, json, Value};

// The query that GraphiQL runs to inspect the schema
static SCHEMA_INTROSPECTION_QUERY: &'static str = r#"
    query IntrospectionQuery {
      __schema {
        queryType {
          name
        }
        mutationType {
          name
        }
        subscriptionType {
          name
        }
        types {
          ...FullType
        }
        directives {
          name
          description
          locations
          args {
            ...InputValue
          }
        }
      }
    }

    fragment FullType on __Type {
      kind
      name
      description
      fields(includeDeprecated: true) {
        name
        description
        args {
          ...InputValue
        }
        type {
          ...TypeRef
        }
        isDeprecated
        deprecationReason
      }
      inputFields {
        ...InputValue
      }
      interfaces {
        ...TypeRef
      }
      enumValues(includeDeprecated: true) {
        name
        description
        isDeprecated
        deprecationReason
      }
      possibleTypes {
        ...TypeRef
      }
    }

    fragment InputValue on __InputValue {
      name
      description
      type {
        ...TypeRef
      }
      defaultValue
    }

    fragment TypeRef on __Type {
      kind
      name
      ofType {
        kind
        name
        ofType {
          kind
          name
          ofType {
            kind
            name
            ofType {
              kind
              name
              ofType {
                kind
                name
                ofType {
                  kind
                  name
                  ofType {
                    kind
                    name
                  }
                }
              }
            }
          }
        }
      }
    }
"#;

graphql_schema_from_file!("tests/schemas/doc_test.graphql");

pub struct Query;

impl QueryFields for Query {
    fn field_query_field<'a>(
        &self,
        _: &Executor<'a, Context>,
        _: InputType,
    ) -> FieldResult<&SomeScalar> {
        unimplemented!()
    }

    fn field_entity<'a>(
        &self,
        _: &Executor<'a, Context>,
        _: &QueryTrail<'a, Entity, Walked>,
    ) -> FieldResult<&Entity> {
        unimplemented!()
    }

    fn field_deprecated_field<'a>(&self, _: &Executor<'a, Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_deprecated_field2<'a>(&self, _: &Executor<'a, Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_search<'a>(
        &self,
        _: &Executor<'a, Context>,
        _: &QueryTrail<'a, SearchResult, Walked>,
        _: String,
    ) -> FieldResult<&Vec<SearchResult>> {
        unimplemented!()
    }
}

pub struct User {
    id: ID,
    user_type: UserType,
}

impl UserFields for User {
    fn field_id<'a>(&self, _: &Executor<'a, Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_user_type<'a>(&self, _: &Executor<'a, Context>) -> FieldResult<&UserType> {
        unimplemented!()
    }
}

pub struct Context;

impl juniper::Context for Context {}

#[test]
fn test_docs() {
    let mut json = introspect_schema()["__schema"]["types"]
        .as_array()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|type_| !type_["name"].as_str().unwrap().starts_with("__"))
        .collect::<Vec<_>>();
    json.sort_by_key(|key| key["name"].as_str().unwrap().to_string());
    let json = serde_json::Value::Array(json);

    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    assert_json_include!(
        actual: json,
        expected:
            json!([
                { "name": "Boolean" },
                {
                    "name": "Entity",
                    "description": "Entity desc",
                    "fields": [
                        {
                            "name": "id",
                            "description": "Entity id desc",
                            "isDeprecated": true,
                            "deprecationReason": null,
                        },
                    ],
                },
                { "name": "ID" },
                {
                    "name": "InputType",
                    "description": "InputType desc",
                    "inputFields": [
                        {
                            "name": "id",
                            "description": "id desc",
                        },
                    ]
                },
                {
                    "name": "Query",
                    "description": "Root query type",
                    "fields": [
                        {
                            "name": "queryField",
                            "description": "queryField desc",
                            "isDeprecated": false,
                            "args": [
                                {
                                    "name": "queryFieldArg",
                                    "description": "queryFieldArg desc",
                                },
                            ],
                        },
                        {
                            "name": "deprecatedField",
                            "description": "deprecatedField desc",
                            "isDeprecated": true,
                            "deprecationReason": null,
                        },
                        {
                            "name": "deprecatedField2",
                            "description": "deprecatedField2 desc",
                            "isDeprecated": true,
                            "deprecationReason": "because reasons",
                        },
                    ],
                },
                {
                    "name": "SearchResult",
                    "description": "SearchResult desc",
                },
                {
                    "name": "SomeScalar",
                    "description": "SomeScalar scalar desc",
                },
                { "name": "String" },
                { "name": "User" },
                {
                    "name": "UserType",
                    "description": "UserType desc",
                    "enumValues": [
                        {
                            "name": "REAL",
                            "description": "REAL desc",
                            "deprecationReason": "because reasons",
                            "isDeprecated": true,
                        },
                        {
                            "name": "BOT",
                            "description": "BOT desc",
                            "deprecationReason": null,
                            "isDeprecated": false,
                        },
                        {
                            "name": "OTHER",
                            "description": "OTHER desc",
                            "deprecationReason": "",
                            "isDeprecated": true,
                        },
                    ],
                },
            ])
    );
}

fn introspect_schema() -> Value {
    let ctx = Context;

    let (juniper_value, _errors) = juniper::execute(
        SCHEMA_INTROSPECTION_QUERY,
        None,
        &Schema::new(Query, juniper::EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    let json: Value =
        serde_json::from_str(&serde_json::to_string(&juniper_value).unwrap()).unwrap();

    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    json
}
