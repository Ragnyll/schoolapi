#[macro_use]
extern crate rocket;

use schoolapi::api;

#[rocket::main]
async fn main() {
    start_api().await.expect("Failed to start api");
}

async fn start_api() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(api::db_connections::SchoolDbConn::fairing())
        .mount("/", routes![api::school_endpoints::all_schools])
        .launch()
        .await?;

    Ok(())
}
