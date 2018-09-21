use query::typedef::*;

pub const QUERY : & 'static str = "query Repository($owner: String!, $name: String!) {\n  repository(owner: $owner, name: $name) {\n    id\n    url\n  }\n}\n" ;
pub const OPERATION_NAME: &'static str = "Repository";
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
pub struct RustRepositoryRepository {
    pub id: ID,
    #[doc = "The HTTP URL for this repository"]
    pub url: URI,
}
#[derive(Serialize, Debug)]
pub struct Variables {
    pub owner: String,
    pub name: String,
}
impl Variables {}
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    #[doc = "Lookup a given repository by the owner and repository name."]
    pub repository: Option<RustRepositoryRepository>,
}

pub struct Repository;

impl<'de> ::graphql_client::GraphQLQuery<'de> for Repository {
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
