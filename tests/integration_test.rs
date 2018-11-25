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

    impl Query {
        pub fn field_string<T>(&self, _: T) -> FieldResult<String> {
            unimplemented!()
        }

        pub fn field_float<T>(&self, _: T) -> FieldResult<f64> {
            unimplemented!()
        }

        pub fn field_int<T>(&self, _: T) -> FieldResult<i32> {
            unimplemented!()
        }

        pub fn field_boolean<T>(&self, _: T) -> FieldResult<bool> {
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

    impl Query {
        pub fn field_string<T>(&self, _: T) -> FieldResult<Option<String>> {
            unimplemented!()
        }

        pub fn field_float<T>(&self, _: T) -> FieldResult<Option<f64>> {
            unimplemented!()
        }

        pub fn field_int<T>(&self, _: T) -> FieldResult<Option<i32>> {
            unimplemented!()
        }

        pub fn field_boolean<T>(&self, _: T) -> FieldResult<Option<bool>> {
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

    impl Query {
        pub fn field_field<T>(&self, _: T) -> FieldResult<Vec<i32>> {
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

    impl Query {
        pub fn field_field<T>(&self, _: T) -> FieldResult<Option<Vec<i32>>> {
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

    impl Query {
        pub fn field_field<T>(&self, _: T) -> FieldResult<Vec<Option<i32>>> {
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

    impl Query {
        pub fn field_field<T>(&self, _: T) -> FieldResult<Option<Vec<Option<i32>>>> {
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

    impl Query {
        pub fn field_field<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<i32> {
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

    impl Query {
        pub fn field_single<T>(&self, executor: T, arg: i32) -> FieldResult<i32> {
            unimplemented!()
        }

        pub fn field_multiple<T>(
            &self,
            executor: T,
            one: i32,
            two: Option<String>,
            three: Option<Vec<Option<f64>>>,
        ) -> FieldResult<i32> {
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

    impl Query {
        pub fn field_yes_no<T>(&self, _: T, arg: Option<YesNo>) -> FieldResult<YesNo> {
            let _: YesNo = YesNo::No;
            let _: YesNo = YesNo::Yes;
            let _: YesNo = YesNo::NotSure;
            unimplemented!()
        }
    }
}
