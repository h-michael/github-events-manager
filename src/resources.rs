use super::schema::repositories;

#[derive(Debug, Queryable)]
pub struct Repository {
    pub id: i32,
    pub owner: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "repositories"]
pub struct NewRepository<'a> {
    pub owner: &'a str,
    pub name: &'a str,
}
