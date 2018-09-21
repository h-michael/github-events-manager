use super::utils::*;
use graphql_client::*;
use query::test;

pub fn token_test() {
    let q = test::Test::build_query(test::Variables {});
    let res = request(&q);

    match res {
        Ok(mut res) => {
            let body: Response<test::ResponseData> = res.json().expect("");
            println!("{:?}", body);
        }
        Err(e) => println!("{:?}", e),
    };
}
