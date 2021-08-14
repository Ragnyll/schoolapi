/// Common db connection operation for managing schools

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::school::{School, NewSchool};
use crate::models::schema::school;

// TODO: have this chain error
pub fn create_school(conn: PgConnection) {
    let new_school = NewSchool {
        name: "street styles",
    };

    diesel::insert_into(school::table)
    .values(&new_school)
    .get_result::<School>(&conn)
    .expect("Error saving new post");
}

