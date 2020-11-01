use user_rust::db::lib::establish_connection;
use user_rust::db::database::create_user;
use user_rust::db::database::create_user_raw;
use user_rust::db::models::{NewUserJson};

fn main(){
    let connection = establish_connection();
    let name = "arthur";
    let comment= "new user here";
    let active = true;
    let password = "Super secret!";



    println!("{:?}", create_user_raw(&connection, name, comment, active, &password));

    let new_user = NewUserJson {
        name : "ford".to_string(),
        comment : "Yay".to_string(),
        active : true,
        password: "passw0rd".to_string()
    };

    println!("{:?}", create_user(&connection, &new_user));

    let new_user2 = NewUserJson {
        name : "trillian".to_string(),
        comment : "Yay".to_string(),
        active : true,
        password: "idiots!".to_string()
    };

    println!("{:?}", create_user(&connection, &new_user2));

}