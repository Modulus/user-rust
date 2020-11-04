use user_rust::db::lib::establish_connection;
use user_rust::db::users::get_all_users;
use user_rust::db::messages::send_message;


fn main(){
    let connection = establish_connection();

    let users = get_all_users(&connection).unwrap();

    let user1 = users.get(0).unwrap();
    let user2 = users.get(1).unwrap();


    send_message(user1, user2, "Hello".to_string(),
                 "First message ever!!! Let's be fiwends".to_string(), &connection);

    send_message(user2, user1, "Hello back at ya!".to_string(), "Aesome stuffs".to_string(), &connection);

}