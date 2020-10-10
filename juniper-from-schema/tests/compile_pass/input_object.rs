#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        usersAtLocation(coordinate: Coordinate): Boolean!
    }

    input Coordinate {
        lat: Int!
        long: Int!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_users_at_location(
        &self,
        executor: &Executor<Context>,
        coordinate: Option<Coordinate>,
    ) -> FieldResult<&bool> {
        unimplemented!()
    }
}
