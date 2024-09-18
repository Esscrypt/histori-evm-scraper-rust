use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;

// Create a type alias for the connection pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}