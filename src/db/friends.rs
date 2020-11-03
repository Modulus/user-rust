use std::str;

use diesel::prelude::*;
use crate::db::models::{User, NewUser, NewUserJson, Friend};
use argon2::{self, Config};
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::errors::BackendError;
use crate::db::users::{get_user_by_name, get_user_by_id};
use crate::utils::lib::date_now;

pub fn add_friend(friends: Friend, conn: &PgConnection) -> Result<String, BackendError>{
    use crate::schema::friends;

    let result  = diesel::insert_into(friends::table)
        .values(&friends)
        .execute(conn)?;

    println!("Result for adding friend was: {:?}", result);

    return Ok("Added".to_string());

}

pub fn list_friends(user: User, conn: &PgConnection) -> Result<Vec<User>, BackendError> {
    use crate::schema::friends;
    use crate::schema::friends::dsl::*;

    let users_friends: Vec<Friend>  = friends.filter(user_id.eq(user.id)).limit(10).load::<Friend>(conn).expect("Error loading fiwends!");

    println!("Listing friends: {:?}", users_friends);

    let mut friend_list : Vec<User> = Vec::new();


    for friend in users_friends {
        let user_from_id = get_user_by_id(conn, friend.friend_id)?;
        println!("Found friend: {:?}", user_from_id);
        friend_list.push(user_from_id);
    }


    return Ok(friend_list)
}


pub fn remove_friend(owner: User, friend: User){

}