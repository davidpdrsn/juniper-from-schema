#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema! {
    type Query {
        string: String! @someDirectiveThatIsntNotSupported
    }

    schema { query: Query }
}

pub struct Query;

impl QueryFields for Query {
    fn field_string<'a>(&self, executor: &Executor<'a, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}
