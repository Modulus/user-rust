use user_rust::db::lib::establish_connection;
use user_rust::db::messages::list_all_messages;
use user_rust::db::users::get_all_users;

fn main() {
    let connection = establish_connection();
    let users = get_all_users(&connection).unwrap();

    for user in users {
        let messages = list_all_messages(&user, &connection).unwrap();
        if messages.len() > 0 {
            println!("{:?}", messages)
        } else {
            println!("User: {:?} has no messages :(", &user)
        }
    }
}
