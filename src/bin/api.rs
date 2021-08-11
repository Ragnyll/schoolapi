#[macro_use]
extern crate rocket;

use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::RunQueryDsl;

use schoolapi::models::school::School;
use schoolapi::models::schema::school::dsl::school;

// Pooled sql connection for the school_db
#[database("postgresql_schooldb")]
struct SchoolDbConn(diesel::PgConnection);


#[get("/schools")]
async fn all_schools(conn: SchoolDbConn) -> &'static str {
    let content = conn.run(|c| school.load::<School>(c)).await.expect("couldnt load");
    println!("{:?}", content);
    "Hello world"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(SchoolDbConn::fairing())
        .mount("/", routes![all_schools])
        .launch()
        .await
        .expect("Unable to start server");
}
