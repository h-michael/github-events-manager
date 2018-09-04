use super::schema::repositories;

#[derive(Debug, Queryable)]
pub struct Repository {
    pub id: i32,
    pub repository_id: String,
    pub owner: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Insertable)]
#[table_name = "repositories"]
pub struct NewRepository<'a> {
    pub owner: &'a str,
    pub repository_id: &'a str,
    pub name: &'a str,
    pub url: &'a str,
}
