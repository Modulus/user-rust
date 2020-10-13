use std::io::{stdin, Read};
use user_rust::db::lib::establish_connection;
use user_rust::db::database::create_user;
use user_rust::db::database::create_user_raw;
use user_rust::db::models::{NewUser, NewUserJson};

fn main(){
    let connection = establish_connection();
    let name = "arthur";
    let comment= "new user here";
    let active = true;
    let password = "Super secret!";



    create_user_raw(&connection, name, comment, active, &password);

    let newUser = NewUserJson {
        name : "ford".to_string(),
        comment : "Yay".to_string(),
        active : true,
        password: "passw0rd".to_string()
    };

    create_user(&connection, newUser);

    let newUser2 = NewUserJson {
        name : "trillian".to_string(),
        comment : "Yay".to_string(),
        active : true,
        password: "idiots!".to_string()
    };

    create_user(&connection, newUser2);

}