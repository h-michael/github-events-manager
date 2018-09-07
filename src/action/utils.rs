use dotenv::dotenv;
use graphql_client::*;
use reqwest;
use serde;
use std::env;

pub fn request<T: serde::Serialize>(
    query: &GraphQLQueryBody<T>,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("https://api.github.com/graphql")
        .header(reqwest::header::Authorization(format!(
            "bearer {}",
            get_token()
        ))).json(query)
        .send()
}

fn get_token() -> String {
    dotenv().ok();
    env::var("GITHUB_ENVENTS_MANAGER_TOKEN").expect("GITHUB_ENVENTS_MANAGER_TOKEN must be set")
}
