extern crate juniper;

use juniper::FieldResult;
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
