use user_rust::db::friends::list_friends;
use user_rust::db::lib::establish_connection;
use user_rust::db::users::get_all_users;

fn main() {
    let connection = establish_connection();
    let users = get_all_users(&connection).unwrap();

    users.into_iter().for_each(|user| {
        let friends = list_friends(&user, &connection);

        match friends {
            Ok(actual_friends) => {
                for friend in actual_friends {
                    println!("Found friend: {:?}", friend);
                }
            }
            _ => {
                println!("Ellol!!!");
            }
        }
    });
}
