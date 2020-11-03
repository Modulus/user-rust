use std::str;

use diesel::prelude::*;
use crate::db::models::{User, NewUser, NewUserJson, NewMessage};
use argon2::{self, Config};
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::errors::BackendError;


pub fn send_message(from: &User, to: &User, _header: String, _contenty: String, conn: &PgConnection){
    let new_message = NewMessage{
        header: _header.to_string(),
        message: _contenty.to_string(),
        sender_user_id: from.id,
        receiver_user_id: to.id
    };

    use crate::schema::messages;
    use crate::schema::messages::dsl::*;

    let result = diesel::insert_into(messages::table).values(new_message).execute(conn);

    println!("Result of insert of message: {:?}", result);
}