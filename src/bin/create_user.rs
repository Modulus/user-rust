use std::io::{stdin, Read};
use user_rust::db::lib::establish_connection;
use user_rust::db::database::create_user;

fn main(){
    let connection = establish_connection();
    let name = "argo";
    let comment= "new user here";
    let active = true;
    let password = "Super secret!";

    create_user(&connection, name, comment, active, password);

}