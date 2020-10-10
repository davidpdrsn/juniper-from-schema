#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
use juniper::{EmptyMutation, Executor, FieldResult, Variables};
use juniper_from_schema::{graphql_schema, graphql_schema_from_file};

pub struct Context;
impl juniper::Context for Context {}

graphql_schema! {
    type Query {
      entities: [Entity!]! @juniper(ownership: "owned")
      search(query: String!): [SearchResult!]! @juniper(ownership: "owned")
    }

    interface Entity {
      id: Int! @juniper(ownership: "owned")
      name: String!
    }

    type User implements Entity {
      id: Int! @juniper(ownership: "owned")
      name: String!
    }

    union SearchResult = User

    schema {
      query: Query
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_entities<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Entity, Walked>,
    ) -> FieldResult<Vec<Entity>> {
        verify_entity_query_trail(trail);
        verify_user_query_trail(&trail.downcast());

        Ok(vec![])
    }

    fn field_search<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, SearchResult, Walked>,
        _query: String,
    ) -> FieldResult<Vec<SearchResult>> {
        verify_search_result_query_trail(trail);
        verify_user_query_trail(&trail.downcast());

        Ok(vec![])
    }
}

fn verify_entity_query_trail(trail: &QueryTrail<Entity, Walked>) {
    if !trail.id() {
        panic!("Entity.id missing from trail")
    }
}

fn verify_search_result_query_trail(trail: &QueryTrail<SearchResult, Walked>) {
    if !trail.id() {
        panic!("id missing from trail")
    }
}

fn verify_user_query_trail(trail: &QueryTrail<User, Walked>) {
    if !trail.id() {
        panic!("User.id missing from trail")
    }
}

pub struct User {
    id: i32,
    name: String,
}

impl UserFields for User {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
        Ok(self.id)
    }

    fn field_name<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
}

#[test]
fn test_converting_interface_trails() {
    query(
        r#"
        query {
            entities {
                id
            }
        }
        "#,
    );
}

#[test]
#[should_panic]
fn test_converting_interface_trails_negative() {
    query(
        r#"
        query {
            entities {
                name
            }
        }
        "#,
    );
}

#[test]
fn test_converting_union_trails() {
    query(
        r#"
        query {
            search(query: "foo") {
                ... on User {
                    id
                }
            }
        }
        "#,
    );
}

#[test]
#[should_panic]
fn test_converting_union_trails_negative() {
    query(
        r#"
        query {
            search(query: "foo") {
                ... on User {
                    name
                }
            }
        }
        "#,
    );
}

fn query(query: &str) {
    let ctx = Context;
    let (juniper_value, _errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, juniper::EmptyMutation::new()),
        &Variables::new(),
        &ctx,
    )
    .unwrap();
}
