use std::env;

// diesel prelude adds the establish_connection to the PgConnection
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
