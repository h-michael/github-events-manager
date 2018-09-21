pub const QUERY: &'static str = "query GrtLoginUser {\n  viewer {\n    login\n  }\n}\n\n";
pub const OPERATION_NAME: &'static str = "Test";
#[allow(dead_code)]
type Boolean = bool;
#[allow(dead_code)]
type Float = f64;
#[allow(dead_code)]
type Int = i64;
#[allow(dead_code)]
type ID = String;
#[derive(Deserialize, Debug)]
#[doc = "A user is an individual\'s account on GitHub that owns repositories and can make new content."]
pub struct RustGrtLoginUserViewer {
    #[doc = "The username used to login."]
    pub login: String,
}
#[derive(Serialize, Debug)]
pub struct Variables;
#[derive(Deserialize, Debug)]
pub struct ResponseData {
    #[doc = "The currently authenticated user."]
    pub viewer: RustGrtLoginUserViewer,
}

pub struct Test;

impl<'de> ::graphql_client::GraphQLQuery<'de> for Test {
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
