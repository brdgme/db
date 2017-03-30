#![recursion_limit = "1024"]

extern crate uuid;
extern crate chrono;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use errors::*;

use r2d2_postgres::{TlsMode, PostgresConnectionManager};

pub mod models;

pub struct Connections {
    pub r: r2d2::Pool<PostgresConnectionManager>,
    pub w: r2d2::Pool<PostgresConnectionManager>,
}

pub fn connect(r_addr: &str, w_addr: &str) -> Result<Connections> {
    Ok(Connections {
           r: sub_conn(r_addr)?,
           w: sub_conn(w_addr)?,
       })
}

fn sub_conn(addr: &str) -> Result<r2d2::Pool<PostgresConnectionManager>> {
    r2d2::Pool::new(r2d2::Config::default(), PostgresConnectionManager::new(addr, TlsMode::None)
        .chain_err(|| "unable to create connection manager")?)
        .chain_err(|| "unable to connect to database")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
