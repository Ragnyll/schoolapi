use std::borrow::Cow;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket_sync_db_pools::diesel::RunQueryDsl;

use super::db_connections;

use crate::models::school::School;
use crate::models::schema::school;

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
        .run(|c| school::dsl::school.load::<School>(c))
        .await
        .expect("couldnt load");
    Some(Json(content))
}

#[get("/schools/<id>")]
pub async fn get_school_by_id(
    conn: db_connections::SchoolDbConn,
    id: i32,
) -> Option<Json<Vec<School>>> {
    // school::table.find(id).first(&conn);
    let content = conn
        //TODO: this move is pretty suspicous
        .run(move |c| school::table.find(id).load::<School>(c))
        .await
        .expect("couldnt load");

    Some(Json(content))
}
