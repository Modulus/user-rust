use diesel::prelude::*;
use crate::db::models::{User, NewUser};
use std::borrow::Borrow;
use sha2::{Digest, Sha512};
use std::str;

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, comment: &'a str, active: bool, password: &'a str) -> User {
    use crate::schema::users;


    let salt = "pepper";
    let mut hasher = Sha512::new();
    hasher.update(&password.as_bytes());
    hasher.update(b"$");
    hasher.update(salt.as_bytes());

    let result = hasher.finalize();

    let hashed_pass = format!("{:x}", result);

    let new_user = NewUser{
        name: name,
        comment: comment,
        active: active,
        pass_hash: &hashed_pass
    };


    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user!")

}

pub fn show_users(conn: &PgConnection){
    use crate::schema::users::dsl::*;
    let result = users.filter(active.eq(true))
        .limit(10)
        .load::<User>(conn)
        .expect("Error loading users!");

    println!("Displaying {}", result.len());
    for user in result {
        println!("User: {:?}", user.name);
        println!("Hash: {:?}", user.pass_hash);
        println!("Comment: {:?}", user.comment);
    }
}