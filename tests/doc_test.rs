#![recursion_limit = "128"]

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate maplit;

use assert_json_diff::assert_json_include;
use juniper::{Executor, FieldResult, Variables, ID};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};
use serde_json::{self, json, Value};
use std::collections::HashMap;

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

graphql_schema! {
    schema {
        query: Query
    }

    "Root query type"
    type Query {
        "queryField desc"
        queryField(
            "queryFieldArg desc"
            queryFieldArg: InputType!
        ): Url!

        entity: Entity!

        search(query: String!): [SearchResult!]!
    }

    "Url scalar desc"
    scalar Url

    "InputType desc"
    input InputType {
        "id desc"
        id: ID!
    }

    type User implements Entity {
        id: ID!
        userType: UserType!
    }

    "Entity desc"
    interface Entity {
        "Entity id desc"
        id: ID!
    }

    "UserType desc"
    enum UserType {
        "REAL desc"
        REAL
        "BOT desc"
        BOT
    }

    "SearchResult desc"
    union SearchResult = User
}

pub struct Query;

impl QueryFields for Query {
    fn field_query_field<'a>(
        &self,
        _: &Executor<'a, Context>,
        _: &QueryTrail<'a, Url, Walked>,
        _: InputType,
    ) -> FieldResult<&Url> {
        unimplemented!()
    }

    fn field_entity<'a>(
        &self,
        _: &Executor<'a, Context>,
        _: &QueryTrail<'a, Entity, Walked>,
    ) -> FieldResult<&Entity> {
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
    let json = introspect_schema();

    assert_json_include!(
        actual: json,
        expected: json!({
            "__schema": {
                "directives": [],
                "queryType": { "name": "Query" },
                "subscriptionType": null,
                "types": [
                    { "name": "Boolean" },
                    { "name": "__InputValue" },
                    { "name": "String" },
                    { "name": "__Field" },
                    {
                        "name": "UserType",
                        "description": "UserType desc",
                        "enumValues": [
                            {
                                "name": "REAL",
                                "description": "REAL desc",
                            },
                            {
                                "name": "BOT",
                                "description": "BOT desc",
                            },
                        ],
                    },
                    { "name": "__TypeKind" },
                    { "name": "__Type" },
                    { "name": "ID" },
                    { "name": "__Schema" },
                    {
                        "name": "Url",
                        "description": "Url scalar desc",
                    },
                    {
                        "name": "Query",
                        "description": "Root query type",
                        "fields": [
                            {
                                "name": "queryField",
                                "description": "queryField desc",
                                "args": [
                                    {
                                        "name": "queryFieldArg",
                                        "description": "queryFieldArg desc",
                                    },
                                ],
                            },
                        ],
                    },
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
                    { "name": "__EnumValue" },
                    {
                        "name": "SearchResult",
                        "description": "SearchResult desc",
                    },
                    { "name": "User" },
                    { "name": "__DirectiveLocation" },
                    { "name": "__Directive" },
                    {
                        "name": "Entity",
                        "description": "Entity desc",
                        "fields": [
                            {
                                "name": "id",
                                "description": "Entity id desc",
                            },
                        ],
                    },
                ],
            }
        })
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
