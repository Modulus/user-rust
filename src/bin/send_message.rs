use user_rust::db::lib::establish_connection;
use user_rust::db::users::create_user;
use user_rust::db::users::get_all_users;
use user_rust::db::friends::add_friend;
use user_rust::db::models::{NewUserJson, Friend, User, NewMessage};
use user_rust::errors::BackendError;
use user_rust::db::messages::send_message;


fn main(){
    let connection = establish_connection();

    let users = get_all_users(&connection).unwrap();

    let user1 = users.get(0).unwrap();
    let user2 = users.get(1).unwrap();

    let header = "Hello".to_string();
    let message = "First message ever!!! Let's be fiwends".to_string();

    send_message(user1, user2, header, message, &connection);

}