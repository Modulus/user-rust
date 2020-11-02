use std::str;

use diesel::prelude::*;
use crate::db::models::{User, NewUser, NewUserJson};
use argon2::{self, Config};
use rand::Rng;
use rand::distributions::Alphanumeric;
use crate::errors::BackendError;


pub fn send_message(){

}