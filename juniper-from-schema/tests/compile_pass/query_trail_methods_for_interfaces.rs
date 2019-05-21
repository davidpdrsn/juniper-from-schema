#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
      posts: [Post!]! @juniper(ownership: "owned")
    }

    type Post {
      comments: [Comment!]! @juniper(ownership: "owned")
    }

    interface Entity {
      id: Int! @juniper(ownership: "owned")
      country: Country! @juniper(ownership: "owned")
    }

    type User implements Entity {
      country: Country! @juniper(ownership: "owned")
      id: Int! @juniper(ownership: "owned")
    }

    type Country {
      id: Int! @juniper(ownership: "owned")
    }

    type Comment {
      author: Entity! @juniper(ownership: "owned")
      id: Int! @juniper(ownership: "owned")
    }

    schema {
      query: Query
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_posts<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Post, Walked>,
    ) -> FieldResult<Vec<Post>> {
        unimplemented!()
    }
}

pub struct Post {
    comments: Vec<Comment>,
}

impl PostFields for Post {
    fn field_comments<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Comment, Walked>,
    ) -> FieldResult<Vec<Comment>> {
        unimplemented!()
    }
}

pub struct Comment {
    id: i32,
}

impl CommentFields for Comment {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
        unimplemented!()
    }

    fn field_author<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Entity, Walked>,
    ) -> FieldResult<Entity> {
        if trail.id() {
            //
        }

        if trail.country().id() {
            //
        }

        unimplemented!()
    }
}

pub struct User {
    id: i32,
}

impl UserFields for User {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
        unimplemented!()
    }

    fn field_country<'a>(
        &self,
        executor: &Executor<'a, Context>,
        trail: &QueryTrail<'a, Country, Walked>,
    ) -> FieldResult<Country> {
        unimplemented!()
    }
}

pub struct Country {
    id: i32,
}

impl CountryFields for Country {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
        unimplemented!()
    }
}
