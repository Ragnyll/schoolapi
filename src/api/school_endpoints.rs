use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel::RunQueryDsl;
use rocket_sync_db_pools::diesel;

use self::diesel::prelude::*;

use super::db_connections;

use crate::models::schema::school;
use crate::models::school::{NewSchool, School};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

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
    let content = conn
        .run(move |c| school::table.find(id).load::<School>(c))
        .await
        .expect("couldnt load");

    Some(Json(content))
}

#[post("/schools", data = "<school>")]
pub async fn create_school(conn: db_connections::SchoolDbConn<'r>, school: NewSchool<'_>) -> Result<()> {
    let new_school = school.clone();
    conn.run( |conn| {
        diesel::insert_into(school::table)
            .values(&new_school)
            .execute(conn)
    })
    .await?;
    Ok(())
}
