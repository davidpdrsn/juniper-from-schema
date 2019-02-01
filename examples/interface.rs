#[macro_use]
extern crate juniper;

use juniper::*;
use juniper_from_schema::graphql_schema;

graphql_schema! {
    schema {
        query: Query
    }

    type Query {
        "#[ownership(owned)]"
        search(query: String!): [SearchResult!]!
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
    id: Id,
    text: String,
}

impl ArticleFields for Article {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&Id> {
        unimplemented!()
    }

    fn field_text(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Tweet {
    id: Id,
    text: String,
}

impl TweetFields for Tweet {
    fn field_id(&self, executor: &Executor<'_, Context>) -> FieldResult<&Id> {
        unimplemented!()
    }

    fn field_text(&self, executor: &Executor<'_, Context>) -> FieldResult<&String> {
        unimplemented!()
    }
}

pub struct Query;

impl QueryFields for Query {
    fn field_search(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, SearchResult, Walked>,
        query: String,
    ) -> FieldResult<Vec<SearchResult>> {
        let article: Article = Article {
            id: Id::new("1"),
            text: "Business".to_string(),
        };
        let tweet: Tweet = Tweet {
            id: Id::new("2"),
            text: "1 weird tip".to_string(),
        };

        let posts = vec![SearchResult::from(article), SearchResult::from(tweet)];

        Ok(posts)
    }
}

fn main() {}
