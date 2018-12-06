extern crate juniper;

use juniper::{Executor, FieldResult};
use juniper_from_schema::graphql_schema;

pub struct Context;
impl juniper::Context for Context {}

mod simple_non_null_scalars {
    use super::*;

    graphql_schema! {
        type Query {
            string: String!
            float: Float!
            int: Int!
            boolean: Boolean!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_string<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
            unimplemented!()
        }

        fn field_float<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&f64> {
            unimplemented!()
        }

        fn field_int<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&i32> {
            unimplemented!()
        }

        fn field_boolean<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&bool> {
            unimplemented!()
        }
    }
}

pub mod simple_nullable_scalars {
    use super::*;

    graphql_schema! {
        type Query {
            string: String
            float: Float
            int: Int
            boolean: Boolean
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_string<'a>(
            &self,
            executor: &Executor<'a, Context>,
        ) -> FieldResult<&Option<String>> {
            unimplemented!()
        }

        fn field_float<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<f64>> {
            unimplemented!()
        }

        fn field_int<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<i32>> {
            unimplemented!()
        }

        fn field_boolean<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<bool>> {
            unimplemented!()
        }
    }
}

pub mod non_null_list_non_null_items {
    use super::*;

    graphql_schema! {
        type Query {
            field: [Int!]!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_field<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Vec<i32>> {
            unimplemented!()
        }
    }
}

pub mod nullable_list_non_null_items {
    use super::*;

    graphql_schema! {
        type Query {
            field: [Int!]
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_field<'a>(
            &self,
            executor: &Executor<'a, Context>,
        ) -> FieldResult<&Option<Vec<i32>>> {
            unimplemented!()
        }
    }
}

pub mod non_null_list_nullable_items {
    use super::*;

    graphql_schema! {
        type Query {
            field: [Int]!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_field<'a>(
            &self,
            executor: &Executor<'a, Context>,
        ) -> FieldResult<&Vec<Option<i32>>> {
            unimplemented!()
        }
    }
}

pub mod nullable_list_nullable_items {
    use super::*;

    graphql_schema! {
        type Query {
            field: [Int]
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_field<'a>(
            &self,
            executor: &Executor<'a, Context>,
        ) -> FieldResult<&Option<Vec<Option<i32>>>> {
            unimplemented!()
        }
    }
}

mod correct_executor_signature {
    use super::*;

    graphql_schema! {
        type Query {
            field: Int!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_field<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&i32> {
            unimplemented!()
        }
    }
}

mod field_args {
    use super::*;

    graphql_schema! {
        type Query {
            single(arg: Int!): Int!
            multiple(one: Int!, two: String, three: [Float]): Int!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_single<'a>(&self, executor: &Executor<'a, Context>, arg: i32) -> FieldResult<&i32> {
            unimplemented!()
        }

        fn field_multiple<'a>(
            &self,
            executor: &Executor<'a, Context>,
            one: i32,
            two: Option<String>,
            three: Option<Vec<Option<f64>>>,
        ) -> FieldResult<&i32> {
            unimplemented!()
        }
    }
}

mod enums {
    use super::*;

    graphql_schema! {
        enum YES_NO {
            YES
            NO
            NOT_SURE
        }

        type Query {
            yesNo(arg: YES_NO): YES_NO!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_yes_no<'a>(
            &self,
            executor: &Executor<'a, Context>,
            arg: Option<YesNo>,
        ) -> FieldResult<YesNo> {
            let _: YesNo = YesNo::No;
            let _: YesNo = YesNo::Yes;
            let _: YesNo = YesNo::NotSure;
            unimplemented!()
        }
    }
}

mod custom_scalar {
    use super::*;

    graphql_schema! {
        scalar Cursor

        type Query {
            field(arg: Cursor!): Cursor!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_field<'a>(
            &self,
            executor: &Executor<'a, Context>,
            arg: Cursor,
        ) -> FieldResult<Cursor> {
            Cursor::new("123".to_string());
            unimplemented!()
        }
    }

    impl Cursor {
        fn new(t: String) -> Self {
            Cursor(t)
        }
    }
}

mod returning_references {
    use super::*;

    graphql_schema! {
        type Query {
            userNullable(id: Int!): User
            userNonNull(id: Int!): User!
        }

        type User {
            id: Int!
            nameNullable: String
            nameNonNull: String!
        }

        schema { query: Query }
    }

    pub struct Query;

    impl QueryFields for Query {
        fn field_user_nullable<'a>(&self, executor: &Executor<'a, Context>, id: i32) -> FieldResult<Option<User>> {
            Ok(find_user(id))
        }

        fn field_user_non_null<'a>(&self, executor: &Executor<'a, Context>, id: i32) -> FieldResult<User> {
            Ok(find_user(id).unwrap())
        }
    }

    struct User {
        id: i32,
        name: String,
        name_nullable: Option<String>,
    }

    impl UserFields for User {
        fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&i32> {
            Ok(&self.id)
        }

        fn field_name_nullable<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&Option<String>> {
            Ok(&self.name_nullable)
        }

        fn field_name_non_null<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
            Ok(&self.name)
        }
    }

    fn find_user(id: i32) -> Option<User> {
        Some(User {
            id,
            name: "Bob".to_string(),
            name_nullable: None,
        })
    }
}
