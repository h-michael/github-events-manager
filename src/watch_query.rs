type X509Certificate = String;
type URI = String;
type HTML = String;
type GitTimestamp = String;
type GitSSHRemote = String;
type GitObjectID = String;
type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries/watch.graphql",
    response_derives = "Debug",
)]
pub struct WatchQuery;
