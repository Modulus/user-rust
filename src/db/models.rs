use crate::schema::friends;
use crate::schema::messages;
use crate::schema::users;
use actix_http::{header, http::header};
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

    pub fn extract_token_from_header_value(header_value: &str) -> Option<String> {
        if header_value.contains("Bearer"){
            match header_value.split_whitespace().last() {
                Some(value) => {
                    return Some(String::from(value).replace("\"", ""));
                }
                None => {
                    return None;
                }
            }


            
        }
        Some(String::from(header_value).replace("\"", ""))
        
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
    use jsonwebtoken::TokenData;

    use crate::db::lib::establish_connection;
    use crate::db::models::NewUserJson;
    use crate::db::models::{TokenHelper, UserLogin};
    
    #[test]
    fn test_validate_has_changed_valid_token_is_valid_is_false(){
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI4NTUwMzksImV4cCI6MTYxMzQ1OTgzOSwibmFtZSI6ImpvaG4ifQ.Uy6BBphzY7GjclDM68nFKhUJfBoYGutdkXMoWZKQBug";
        let modified = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI4NTUwMzksImV4cCI6MTYxMzQ1OTgzOSwibmFtZSI6ImpvaG4ifQ.Uy6BBphzY7GjclDM68nFKhUJfBoYGutdkXMoWZKQBUG");

        assert!(TokenHelper::validate_token(&modified) == false);
    }


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

    #[test]
    fn test_extract_token_from_header_value_has_expected_format_returns_expected_string(){
        let authorization ="Bearer \"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI0NzM1MDAsImV4cCI6MTYxMzA3ODMwMCwidXNlciI6ImpvaG4ifQ.aBe4D5uFpKEXF_QjRrfyLIP6qdS4glQM1Ty-yc2bOXk\"";

        let result = TokenHelper::extract_token_from_header_value(authorization);

        assert_eq!(String::from(result.unwrap()), String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI0NzM1MDAsImV4cCI6MTYxMzA3ODMwMCwidXNlciI6ImpvaG4ifQ.aBe4D5uFpKEXF_QjRrfyLIP6qdS4glQM1Ty-yc2bOXk"))
    }

    #[test]
    fn test_extract_token_from_header_has_just_token_returns_same_token(){
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI0NzM1MDAsImV4cCI6MTYxMzA3ODMwMCwidXNlciI6ImpvaG4ifQ.aBe4D5uFpKEXF_QjRrfyLIP6qdS4glQM1Ty-yc2bOXk";

        let result = TokenHelper::extract_token_from_header_value(token);

        assert_eq!(result.unwrap(), String::from(token));
    }


    #[test]
    fn test_extract_token_from_header_value_has_quotes_returns_string_without_quotes(){
        let token = "\"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI0NzM1MDAsImV4cCI6MTYxMzA3ODMwMCwidXNlciI6ImpvaG4ifQ.aBe4D5uFpKEXF_QjRrfyLIP6qdS4glQM1Ty-yc2bOXk\"";

        let result = TokenHelper::extract_token_from_header_value(token);

        assert_eq!(result.unwrap(), String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTI0NzM1MDAsImV4cCI6MTYxMzA3ODMwMCwidXNlciI6ImpvaG4ifQ.aBe4D5uFpKEXF_QjRrfyLIP6qdS4glQM1Ty-yc2bOXk"));
    }


}
