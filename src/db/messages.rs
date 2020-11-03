use std::str;

use diesel::prelude::*;
use crate::db::models::{User, NewUser, NewUserJson, NewMessage, Message};
use crate::utils::lib::date_now;
use argon2::{self, Config};
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::errors::BackendError;


pub fn send_message(from: &User, to: &User, _header: String, _contenty: String, conn: &PgConnection){
    let new_message = NewMessage{
        header: _header,
        message: _contenty,
        sender_user_id: from.id,
        receiver_user_id: to.id,
        sent: date_now(),
        modified: None
    };

    use crate::schema::messages;
    use crate::schema::messages::dsl::*;

    let result = diesel::insert_into(messages::table).values(new_message).execute(conn);

    println!("Result of insert of message: {:?}", result);
}


pub fn list_messages(user: &User, conn: &PgConnection) -> Result<Vec<Message>, BackendError>{
    use crate::schema::messages;
    use crate::schema::messages::dsl::*;
    use crate::schema::users;
    use crate::schema::users::dsl::*;


    let result = messages.filter(sender_user_id.eq(user.id)).limit(25).load::<Message>(conn)?;
    return Ok(result);


}

pub fn change_message(from: &User, to: &User, new_header: String, new_content: String, conn: &PgConnection){
    unimplemented!("Not impelemented yet")
}