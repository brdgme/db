#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate uuid;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

mod schema;
mod models;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
