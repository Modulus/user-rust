use diesel::prelude::*;
use crate::db::models::{User, NewUser};
use sha2::digest::DynDigest;
use sha2::Digest;
use std::borrow::Borrow;

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, comment: &'a str, active: bool, password: &'a str) -> User {
    use crate::schema::users;



    // let mut hasher = Sha256::new();
    // hasher.input_str(password);
    // let hex = hasher.result_str();

    let new_user = NewUser{
        name: name,
        comment: comment,
        active: active,
        pass_hash: password
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