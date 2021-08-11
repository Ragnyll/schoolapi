extern crate diesel;
extern crate schoolapi;

use self::schoolapi::*;
use self::thing::*;
use self::diesel::prelude::*;

fn main() {
    use schoolapi::models::schema::school::dsl::*;
    use schoolapi::models::schema::school;

    let connection = establish_connection();

    let new_school = NewSchool {
    name: "street styles",
    };

    diesel::insert_into(school::table)
    .values(&new_school)
    .get_result::<School>(&connection)
    .expect("Error saving new post");

    let results = school
        .limit(5)
        .load::<School>(&connection)
        .expect("Error loading school");

    println!("Displaying {} schools", results.len());

    for result in results {
        println!("{}", result.id);
        println!("----------\n");
        println!("{}", result.name);
    }
}
