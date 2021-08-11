#[macro_use]
extern crate rocket;

use schoolapi::errors::*;

use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::RunQueryDsl;

use schoolapi::models::school::School;
use schoolapi::models::schema::school::dsl::school;

// Pooled sql connection for the school_db
#[database("postgresql_schooldb")]
struct SchoolDbConn(diesel::PgConnection);


#[get("/schools")]
async fn all_schools(conn: SchoolDbConn) -> &'static str {
    let content = conn.run(|c| school.load::<School>(c)).await.expect("couldnt load");
    println!("{:?}", content);
    "Hello world"
}

#[rocket::main]
async fn main() {
    if let Err(ref e) = start_api().await {
            use error_chain::ChainedError;
            use std::io::Write; // trait which holds `display_chain`
            let stderr = &mut ::std::io::stderr();
            let errmsg = "Error writing to stderr";

            writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
            ::std::process::exit(1);
    }
}

async fn start_api() -> Result<()> {
    rocket::build()
        .attach(SchoolDbConn::fairing())
        .mount("/", routes![all_schools])
        .launch()
        .await.chain_err(|| "unable to open start api")?;

    Ok(())
}
