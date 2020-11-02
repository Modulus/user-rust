use std::str;

use diesel::prelude::*;
use crate::db::models::{User, NewUser, NewUserJson, NewFriend};
use argon2::{self, Config};
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::errors::BackendError;

pub fn add_friend(friends: NewFriend, conn: &PgConnection) -> Result<String, BackendError>{
    use crate::schema::friends;

    let result  = diesel::insert_into(friends::table)
        .values(&friends)
        .execute(conn)?;

    println!("Result for adding friend was: {:?}", result);

    return Ok("Added".to_string());

}


pub fn remove_friend(owner: User, friend: User){

}