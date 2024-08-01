use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&url).unwrap_or_else(|_| panic!("Error conecting to {}", url))
}
