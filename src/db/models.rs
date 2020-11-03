use crate::schema::users;
use crate::schema::friends;
use crate::schema::messages;
use serde::{Serialize, Deserialize};

//TODO: Add date types for all models


#[table_name= "users"]
#[derive(Insertable, Debug, Serialize, Queryable, Clone)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub active: bool,
    pub pass_hash: String
}

#[derive(Insertable,Debug, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub comment: &'a str,
    pub active: bool,
    pub pass_hash: &'a str // Used for the password string, needs same name to be serializable
}

#[table_name= "friends"]
#[derive(Insertable, Debug, Serialize, Queryable)]
pub struct Friend {
    pub user_id: i32,
    pub friend_id: i32
}


#[derive(Insertable, Debug, Serialize, Queryable)]
#[table_name = "messages"]

pub struct Message {
    pub id: i32,
    pub header: String,
    pub message: String,
    pub sender_user_id: i32,
    pub receiver_user_id: i32
}


#[derive(Insertable, Debug, Serialize, Queryable)]
#[table_name = "messages"]
pub struct NewMessage {
    pub header: String,
    pub message: String,
    pub sender_user_id: i32,
    pub receiver_user_id: i32
}



// Model for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub id: i32,
    pub name: String,
    pub comment: Option<String>,
    pub active: bool,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserJson {
    pub name: String,
    pub comment: String,
    pub active: bool,
    pub password: String
}

