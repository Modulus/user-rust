extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;

fn main() {
    let database_url = "postgres://user:user@localhost/user".to_string();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::new(manager).unwrap();
    let conn = pool.get();


}