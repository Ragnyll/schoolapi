use std::borrow::Cow;

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket_sync_db_pools::diesel::RunQueryDsl;

use super::db_connections;

use crate::models::school::School;
use crate::models::schema::school::dsl::school;

// The type to represent the ID of a message.
type Id = usize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    id: Option<Id>,
    message: Cow<'r, str>,
}

#[get("/schools")]
pub async fn all_schools(conn: db_connections::SchoolDbConn) -> Option<Json<Vec<School>>> {
    let content = conn
        .run(|c| school.load::<School>(c))
        .await
        .expect("couldnt load");
    println!("{:?}", content);
    Some(Json(content))
}
