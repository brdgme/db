#![recursion_limit = "1024"]

extern crate uuid;
extern crate chrono;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use]
extern crate error_chain;
extern crate rand;
#[macro_use]
extern crate lazy_static;

pub mod errors {
    error_chain!{
        foreign_links {
            Postgres(::postgres::error::Error);
            EnvVar(::std::env::VarError);
        }
    }
}
pub mod query;
pub mod models;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use std::env;
use errors::*;

pub struct Connections {
    pub w: r2d2::Pool<PostgresConnectionManager>,
    pub r: r2d2::Pool<PostgresConnectionManager>,
}

pub fn connect(w_addr: &str, r_addr: &str) -> Result<Connections> {
    Ok(Connections {
           w: conn(w_addr)?,
           r: conn(r_addr)?,
       })
}

pub fn connect_env() -> Result<Connections> {
    let w_addr = env::var("DATABASE_URL").chain_err(|| "DATABASE_URL not set")?;
    connect(&w_addr,
            &env::var("DATABASE_URL_R").unwrap_or(w_addr.to_owned()))
}

fn conn(addr: &str) -> Result<r2d2::Pool<PostgresConnectionManager>> {
    r2d2::Pool::new(r2d2::Config::default(), PostgresConnectionManager::new(addr, TlsMode::None)
        .chain_err(|| "unable to create connection manager")?)
        .chain_err(|| "unable to connect to database")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
