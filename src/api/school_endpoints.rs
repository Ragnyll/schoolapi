use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::RunQueryDsl;

use crate::models::school::School;
use crate::models::schema::school::dsl::school;

// Pooled sql connection for the school_db
#[database("postgresql_schooldb")]
pub struct SchoolDbConn(diesel::PgConnection);

#[get("/schools")]
pub async fn all_schools(conn: SchoolDbConn) -> &'static str {
    let content = conn.run(|c| school.load::<School>(c)).await.expect("couldnt load");
    println!("{:?}", content);
    "Hello world"
}
