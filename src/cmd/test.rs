use super::utils::*;
use graphql_client::*;
use crate::query::login_user;

pub fn token_test() {
    let q = login_user::LoginUser::build_query(login_user::Variables {});
    let res = request(&q);

    match res {
        Ok(mut res) => {
            let body: Response<login_user::ResponseData> = res.json().expect("");
            println!("{:?}", body);
        }
        Err(e) => println!("{:?}", e),
    };
}
