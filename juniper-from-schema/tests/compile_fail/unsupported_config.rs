#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("../setup.rs");

graphql_schema_from_file!(
    "../../../juniper-from-schema/tests/schemas/customizing_context_name.graphql",
    foo: Foo
);

pub struct Query;

impl QueryFields for Query {
    fn field_foo<'a>(&self, _: &Executor<'a, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}
