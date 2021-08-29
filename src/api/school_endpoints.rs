use rocket::response::Debug;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel::RunQueryDsl;
use rocket_sync_db_pools::diesel;

use self::diesel::prelude::*;

use super::db_connections;

use crate::models::school::School;
use crate::models::schema::school;

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
pub async fn create_school(conn: db_connections::SchoolDbConn, school: Json<School>) -> Result<Created<Json<School>>> {
    let school_value = school.clone();
    conn.run(move |conn| {
        diesel::insert_into(school::table)
            .values(&school_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/").body(school))
}
