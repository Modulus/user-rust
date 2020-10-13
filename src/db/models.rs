use crate::schema::users;
use diesel::prelude::*;
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::{serialize, deserialize};
use serde::{Serialize, Deserialize};


#[table_name= "users"]
#[derive(Insertable, Debug, Serialize, Queryable)]
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

// Model for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub id: i32,
    pub name: String,
    pub comment: String,
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