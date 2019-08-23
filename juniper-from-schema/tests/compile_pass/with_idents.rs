#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

use juniper::ID;

graphql_schema_from_file!(
    "../../../juniper-from-schema/tests/schemas/complex_schema.graphql",
    with_idents: [Character, Droid, Human],
    error_type: MyError,
);

pub enum MyError {
    Foo,
    Bar,
}
impl juniper::IntoFieldError for MyError {
    fn into_field_error(self) -> juniper::FieldError {
        unimplemented!()
    }
}

pub struct Human { }

impl HumanFields for Human {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> Result<ID, MyError> {
        unimplemented!()
    }
    fn field_name<'a>(&self, executor: &Executor<'a, Context>) -> Result<&String, MyError> {
        unimplemented!()
    }
}

pub struct Droid { }

impl DroidFields for Droid {
    fn field_id<'a>(&self, executor: &Executor<'a, Context>) -> Result<ID, MyError> {
        unimplemented!()
    }
    fn field_name<'a>(&self, executor: &Executor<'a, Context>) -> Result<&String, MyError> {
        unimplemented!()
    }
}