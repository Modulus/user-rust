use crate::schema::users;
use diesel::prelude::*;
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::{serialize, deserialize};
use serde::{Serialize};


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
    pub pass_hash: &'a str
}