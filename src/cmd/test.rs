use super::utils::*;
use graphql_client::*;
use query::test;

pub fn token_test() {
    let q = test::TestQuery::build_query(test::test_query::Variables {});
    let res = request(&q);

    match res {
        Ok(mut res) => {
            let body: GraphQLResponse<test::test_query::ResponseData> = res.json().expect("");
            println!("{:?}", body);
        }
        Err(e) => println!("{:?}", e),
    };
}
