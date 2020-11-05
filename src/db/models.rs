use crate::schema::friends;
use crate::schema::messages;
use crate::schema::users;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
//TODO: Add date types for all models
use jsonwebtoken::{EncodingKey, Header};

#[macro_use]
use diesel::prelude::*;

#[table_name = "users"]
#[derive(Insertable, Debug, Serialize, Queryable, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub active: bool,
    pub pass_hash: String, //Needs to be option to be excluded inn some read calls
    pub created: NaiveDateTime,
}

// #[table_name = "users"]
// #[derive(Debug, Serialize, Queryable, Clone, PartialEq, Eq)]
// pub struct UserSafe {
//     pub id: i32,
//     pub name: String,
//     pub comment: Option<String>,
//     pub created: NaiveDateTime,
// }

#[derive(Insertable, Debug, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub comment: &'a str,
    pub active: bool,
    pub pass_hash: &'a str, // Used for the password string, needs same name to be serializable
    pub created: NaiveDateTime,
}

#[table_name = "friends"]
#[derive(Insertable, Debug, Serialize, Deserialize, Queryable)]
pub struct Friend {
    pub user_id: i32,
    pub friend_id: i32,
    pub added: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendJson {
    pub user_id: i32,
    pub friend_id: i32,
}

#[derive(Insertable, Debug, Serialize, Queryable)]
#[table_name = "messages"]
pub struct Message {
    pub id: i32,
    pub header: String,
    pub message: String,
    pub sender_user_id: i32,
    pub receiver_user_id: i32,
    pub sent: NaiveDateTime,
    pub modified: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Serialize, Deserialize, Queryable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub header: String,
    pub message: String,
    pub sender_user_id: i32,
    pub receiver_user_id: i32,
    pub sent: NaiveDateTime,
    pub modified: Option<NaiveDateTime>,
}

// Model for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub active: bool,
    pub password: String,
    // pub create: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserJson {
    pub name: String,
    pub comment: String,
    pub active: bool,
    pub password: String,
    // pub created: chrono::NaiveDateTime
}

// Model for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub password: String,
}

// pub static KEY: [u8; 16] = *include_bytes!("../secret.key");

// pub static KEY: [u8, 16] = b"My secret key012";

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    //issued at
    pub iat: i64,
    //expiration
    pub exp: i64,
    //data
    pub user: String,
    // pub login_session: String,
}

impl JwtToken {
    pub fn generate_token(login: &UserLogin) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = JwtToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.name.clone(),
            // login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret("Secret which should be in rilfe".as_ref()),
            // &EncodingKey::from_secret(&KEY),
        )
        .unwrap()
    }
}
