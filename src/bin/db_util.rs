/// A collection of commands cli interaction for the database

extern crate diesel;
extern crate schoolapi;

use self::diesel::prelude::*;

#[path="db_util_cli/cli_parser.rs"]
mod cli_parser;
use cli_parser::DBOperation;

use self::schoolapi::establish_connection;
use self::schoolapi::models::school::{School, NewSchool};

fn main() {
    let operation_type = cli_parser::parse_line();
    match operation_type {
        DBOperation::NewRowOp(ot) => println!("im gonna create a new row with the operation {:?}", ot),
        DBOperation::ListAllTables => println!("im gonna list all the tables"),
    }


    // use schoolapi::models::schema::school::dsl::*;
    // use schoolapi::models::schema::school;

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
}
