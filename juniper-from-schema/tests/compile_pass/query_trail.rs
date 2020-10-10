#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

juniper_from_schema::graphql_schema! {
    type Query {
        user: User!
    }

    type User {
        id: Int!
        club: Club
        club2: Club!
    }

    type Club {
        id: Int!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_user(
        &self,
        executor: &Executor<Context>,
        trail: &QueryTrail<User, Walked>,
    ) -> FieldResult<&User> {
        trail.club().walk();
        trail.club2().walk();
        trail.club2().id() == true;

        unimplemented!()
    }
}

pub struct User {
    id: i32,
}

impl UserFields for User {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&i32> {
        unimplemented!()
    }

    fn field_club(
        &self,
        executor: &Executor<Context>,
        trail: &QueryTrail<Club, Walked>,
    ) -> FieldResult<&Option<Club>> {
        unimplemented!()
    }

    fn field_club2(
        &self,
        executor: &Executor<Context>,
        trail: &QueryTrail<Club, Walked>,
    ) -> FieldResult<&Club> {
        unimplemented!()
    }
}

pub struct Club {
    id: i32,
}

impl ClubFields for Club {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&i32> {
        unimplemented!()
    }
}
