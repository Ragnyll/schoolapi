#[macro_use]
extern crate rocket;

use schoolapi::schema::school::dsl::*;
use self::diesel::prelude::*;

use rocket_sync_db_pools::{diesel, database};
use schoolapi::schema::school::dsl::*;
use schoolapi::schema::school;
use schoolapi::models::School;

#[database("postgresql_schooldb")]
struct SchoolDbConn(diesel::PgConnection);

use rocket_sync_db_pools::diesel::RunQueryDsl;

#[get("/schools")]
async fn all_schools(conn: SchoolDbConn) -> &'static str {
    let content = conn.run(|c| school.load::<School>(c)).await.expect("couldnt load");
    println!("{:?}", content);

    // let connection = establish_connection();

    // let new_school = NewSchool {
    // name: "street styles",
    // };

    // diesel::insert_into(school::table)
    // .values(&new_school)
    // .get_result::<School>(&connection)
    // .expect("Error saving new post");

    // let results = school
        // .limit(5)
        // .load::<School>(&connection)
        // .expect("Error loading school");

    // println!("Displaying {} schools", results.len());

    // for result in results {
        // println!("{}", result.id);
        // println!("----------\n");
        // println!("{}", result.name);
    // }
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
