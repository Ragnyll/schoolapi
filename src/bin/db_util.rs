/// A collection of commands cli interaction for the database
extern crate diesel;
extern crate schoolapi;

use schoolapi::db::{common, school};

#[path = "db_util_cli/cli_parser.rs"]
mod cli_parser;
use cli_parser::DBOperation;

fn main() {
    let conn = common::establish_connection();
    let operation = cli_parser::parse_line();

    match operation {
        DBOperation::NewRowOp(_) => {
            school::create_school(conn).expect("Failed to create new row in table")
        }
        DBOperation::SelectTable(_) => println!("im gonna select all the tables"),
        DBOperation::ListAllTables => println!("im gonna list all the tables"),
    };
}
