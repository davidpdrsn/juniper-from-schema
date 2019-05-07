#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
      "#[ownership(owned)]"
      posts: [Post!]!
    }

    type Post {
      "#[ownership(owned)]"
      comments: [Comment!]!
    }

    union Entity = User | Company

    type User {
      "#[ownership(owned)]"
      country: Country!
      "#[ownership(owned)]"
      id: Int!
    }

    type Company {
      "#[ownership(owned)]"
      country_of_operation: Country!
      "#[ownership(owned)]"
      id: Int!
      "#[ownership(owned)]"
      name: String!
    }

    type Country {
      "#[ownership(owned)]"
      id: Int!
    }

    type Comment {
      "#[ownership(owned)]"
      author: Entity!
      "#[ownership(owned)]"
      id: Int!
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
        let _: bool = trail.id();
        let _: bool = trail.country().id();
        let _: QueryTrail<'a, Country, NotWalked> = trail.country();
        let _: bool = trail.country_of_operation().id();
        let _: QueryTrail<'a, Country, NotWalked> = trail.country_of_operation();
        let _: bool = trail.name();

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

pub struct Company {
    id: i32,
}

impl CompanyFields for Company {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
        unimplemented!()
    }

    fn field_name<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<String> {
        unimplemented!()
    }

    fn field_country_of_operation<'a>(
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
