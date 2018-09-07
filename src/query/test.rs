use query::typedef::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/test.graphql",
    response_derives = "Debug",
)]
pub struct TestQuery;
