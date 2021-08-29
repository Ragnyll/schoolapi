use super::schema::school;
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "school"]
pub struct School {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}
#[derive(Clone, FromForm, Insertable)]
#[table_name = "school"]
pub struct NewSchool<'a> {
    pub name: &'a str,
}
