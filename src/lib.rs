#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate uuid;
extern crate chrono;

pub mod schema;
pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
