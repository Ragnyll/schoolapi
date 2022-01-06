use rocket_sync_db_pools::database;

// Pooled sql connection for the school_db
#[database("postgresql_schooldb")]
pub struct SchoolDbConn(diesel::PgConnection)<'r>;
