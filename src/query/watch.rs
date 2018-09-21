use query::typedef::*;

pub const QUERY : &str = "query WatchingRepositories($first: Int!, $after: String){\n  viewer {\n    watching(after: $after, first: $first) {\n      nodes {\n        id,\n        nameWithOwner,\n        url\n      }\n      pageInfo {\n        hasNextPage,\n        endCursor\n      }\n    }\n  }\n}\n" ;
pub const OPERATION_NAME: &str = "WatchQuery";
#[allow(dead_code)]
type Boolean = bool;
#[allow(dead_code)]
type Float = f64;
#[allow(dead_code)]
type Int = i64;
#[allow(dead_code)]
type ID = String;
#[doc = "An RFC 3986, RFC 3987, and RFC 6570 (level 4) compliant URI string."]
#[derive(Deserialize, Debug)]
#[doc = "A repository contains the content for a project."]
pub struct RustWatchingRepositoriesViewerWatchingNodes {
    pub id: ID,
    #[doc = "The repository\'s name with owner."]
    #[serde(rename = "nameWithOwner")]
    pub name_with_owner: String,
    #[doc = "The HTTP URL for this repository"]
    pub url: URI,
}
#[derive(Deserialize, Debug)]
#[doc = "Information about pagination in a connection."]
pub struct RustWatchingRepositoriesViewerWatchingPageInfo {
    #[doc = "When paginating forwards, are there more items?"]
    #[serde(rename = "hasNextPage")]
    pub has_next_page: Boolean,
    #[doc = "When paginating forwards, the cursor to continue."]
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}
#[derive(Deserialize, Debug)]
#[doc = "A list of repositories owned by the subject."]
pub struct RustWatchingRepositoriesViewerWatching {
    #[doc = "A list of nodes."]
    pub nodes: Option<Vec<Option<RustWatchingRepositoriesViewerWatchingNodes>>>,
    #[doc = "Information to aid in pagination."]
    #[serde(rename = "pageInfo")]
    pub page_info: RustWatchingRepositoriesViewerWatchingPageInfo,
}
#[derive(Deserialize, Debug)]
#[doc = "A user is an individual\'s account on GitHub that owns repositories and can make new content."]
pub struct RustWatchingRepositoriesViewer {
    #[doc = "A list of repositories the given user is watching."]
    pub watching: RustWatchingRepositoriesViewerWatching,
}
#[derive(Serialize, Debug)]
pub struct Variables {
    pub first: Int,
    pub after: Option<String>,
}
impl Variables {}
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    #[doc = "The currently authenticated user."]
    pub viewer: RustWatchingRepositoriesViewer,
}

pub struct WatchQuery;

impl<'de> ::graphql_client::GraphQLQuery<'de> for WatchQuery {
    type Variables = Variables;
    type ResponseData = ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        ::graphql_client::QueryBody {
            variables,
            query: QUERY,
            operation_name: OPERATION_NAME,
        }
    }
}
