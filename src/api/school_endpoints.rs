use rocket_sync_db_pools::diesel::RunQueryDsl;

use super::db_connections;

use crate::models::school::School;
use crate::models::schema::school::dsl::school;

#[get("/schools")]
pub async fn all_schools(conn: db_connections::SchoolDbConn) -> &'static str {
    let content = conn.run(|c| school.load::<School>(c)).await.expect("couldnt load");
    println!("{:?}", content);
    "Hello world"
}
