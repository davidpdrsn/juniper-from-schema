#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

use juniper_from_schema::*;
use url::Url;

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        find(id: ID!): User! @juniper(async: true)
    }

    type User {
        id: ID! @juniper(infallible: true)
    }
}

pub struct Query;

#[juniper_from_schema::juniper::async_trait]
impl QueryFields for Query {
    async fn field_find<'s, 'r, 'a>(
        &'s self,
        _: &Executor<'r, 'a, Context>,
        trail: &QueryTrail<'r, User, Walked>,
        id: ID,
    ) -> FieldResult<&'s User> {
        todo!()
    }
}

#[derive(Debug)]
pub struct User {
    id: ID,
}

impl UserFields for User {
    fn field_id(&self, _: &Executor<Context>) -> &ID {
        todo!()
    }
}
