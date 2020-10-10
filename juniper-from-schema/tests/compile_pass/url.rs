#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]
include!("setup.rs");

use url::Url;

juniper_from_schema::graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        url: Url! @juniper(ownership: "owned")
    }

    scalar Url
}

pub struct Query;

impl QueryFields for Query {
    fn field_url(&self, _: &Executor<Context>) -> FieldResult<Url> {
        let url = Url::parse("https://example.com").unwrap();
        Ok(url)
    }
}
