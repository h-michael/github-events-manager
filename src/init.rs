use db_utils::*;

embed_migrations!("migrations");
pub fn init() {
    let connection = establish_connection();
    embedded_migrations::run(&connection).expect("migration error occured");
}
