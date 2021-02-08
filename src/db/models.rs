use crate::schema::friends;
use crate::schema::messages;
use crate::schema::users;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
//TODO: Add date types for all models
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{EncodingKey, Header, Validation, DecodingKey};
use log::{error};
// #[macro_use]
// use diesel::prelude::*;

#[table_name = "users"]
#[derive(Insertable, Debug, Serialize, Queryable, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub active: bool,
    #[serde(skip_serializing)]
    pub pass_hash: String, //Needs to be option to be excluded inn some read calls
    pub created: NaiveDateTime,
}


#[derive(Insertable, Debug, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub comment: &'a str,
    pub active: bool,
    #[serde(skip_serializing)]
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
pub struct Claims {
    //issued at
    pub iat: i64,
    //expiration
    pub exp: i64,
    //data
    pub name: String,
    // pub login_session: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenHelper {

}

impl TokenHelper {
    pub fn generate_token(login: &UserLogin) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = Claims {
            iat: now,
            exp: now + ONE_WEEK,
            name: login.name.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret("Secret which should be in rilfe".as_ref()),
        )
        .unwrap()

    }

    pub fn validate_token(token: &String) -> bool {
        match jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret("Secret which should be in rilfe".as_ref()), &Validation::default()) {
            Ok(_c) => true,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    error!("Token is invalid");
                    false
                }, // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => {
                    error!("Issuer is invalid");
                    false
                }, // Example on how to handle a specific error
                _ => {
                    error!("Some other errors"); 
                    false
                }
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::db::lib::establish_connection;
    use crate::db::models::NewUserJson;
    use crate::db::models::{TokenHelper, UserLogin};
    

    #[test]
    fn test_validate_has_valid_token_is_valid() {

        let user_login = UserLogin{
            name: String::from("User"),
            password: String::from("Pass")
        };

        let token = TokenHelper::generate_token(&user_login);

        assert!(token.len() > 0);
        println!("{}", token);


        assert!(TokenHelper::validate_token(&token))
    }

    #[test]
    fn test_valid_has_altered_valid_token_is_invalid(){
        let user_login = UserLogin{
            name: String::from("User"),
            password: String::from("Pass")
        };

        let token = TokenHelper::generate_token(&user_login);

        assert!(token.len() > 0);

        let mut new_token = "".to_owned();
        new_token.push_str(&token);
        new_token.push_str("asdf");

        assert!(TokenHelper::validate_token(&new_token) == false);


    }

    #[test]
    fn test_valid_has_gibberish_token_is_invalid(){

        let token = String::from("Dette er bare drit!");
        assert!(TokenHelper::validate_token(&token) == false);
    }


}
