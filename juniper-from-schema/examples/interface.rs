#![allow(dead_code, unused_variables, unused_imports)]

use juniper::*;
use juniper_from_schema::graphql_schema;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        search(query: String!): [SearchResult!]! @juniper(ownership: "owned")
    }

    interface SearchResult {
        id: ID!
        text: String!
    }

    type Article implements SearchResult {
        id: ID!
        text: String!
    }

    type Tweet implements SearchResult {
        id: ID!
        text: String!
    }
}

pub struct Context;

impl juniper::Context for Context {}

pub struct Article {
    id: ID,
    text: String,
}

impl ArticleFields for Article {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_text(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Tweet {
    id: ID,
    text: String,
}

impl TweetFields for Tweet {
    fn field_id(&self, executor: &Executor<Context>) -> FieldResult<&ID> {
        unimplemented!()
    }

    fn field_text(&self, executor: &Executor<Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_search(
        &self,
        executor: &Executor<Context>,
        trail: &QueryTrail<SearchResult, Walked>,
        query: String,
    ) -> FieldResult<Vec<SearchResult>> {
        let article: Article = Article {
            id: ID::new("1"),
            text: "Business".to_string(),
        };
        let tweet: Tweet = Tweet {
            id: ID::new("2"),
            text: "1 weird tip".to_string(),
        };

        let posts = vec![SearchResult::from(article), SearchResult::from(tweet)];

        Ok(posts)
    }
}

fn main() {}
