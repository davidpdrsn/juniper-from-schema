#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

mod schema {
    use super::*;

    juniper_from_schema::graphql_schema! {
        type Query {
            usersAtLocation(coordinate: Coordinate!): Boolean!
        }

        input Coordinate {
            lat: Int!
            long: Int!
        }

        schema { query: Query }
    }
}

pub struct Query;

impl schema::QueryFields for Query {
    fn field_users_at_location<'a>(
        &self,
        executor: &Executor<'a, Context>,
        coordinate: schema::Coordinate,
    ) -> FieldResult<&bool> {
        coordinate.lat;
        coordinate.long;

        unimplemented!()
    }
}
