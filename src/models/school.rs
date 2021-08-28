use super::schema::school;
use rocket::serde::{Serialize};

#[derive(Queryable, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct School {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "school"]
pub struct NewSchool<'a> {
    pub name: &'a str,
}
