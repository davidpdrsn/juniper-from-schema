#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
        user: User!
    }

    type User {
        id: Int!
        club: Club
        club_2: Club!
    }

    type Club {
        id: Int!
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_user<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, User, Walked>,
    ) -> FieldResult<&User> {
        trail.club().walk();
        trail.club_2().walk();
        trail.club_2().id() == true;

        unimplemented!()
    }
}

pub struct User {
    id: i32,
}

impl UserFields for User {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&i32> {
        unimplemented!()
    }

    fn field_club<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Club, Walked>,
    ) -> FieldResult<&Option<Club>> {
        unimplemented!()
    }

    fn field_club_2<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Club, Walked>,
    ) -> FieldResult<&Club> {
        unimplemented!()
    }
}

pub struct Club {
    id: i32,
}

impl ClubFields for Club {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&i32> {
        unimplemented!()
    }
}
