use user_rust::db::friends::add_friend_by_id;
use user_rust::db::lib::establish_connection;
use user_rust::db::models::Friend;
use user_rust::db::users::get_all_users;
use user_rust::utils::lib::date_now;

fn main() {
    let connection = establish_connection();

    match get_all_users(&connection) {
        Ok(users) => {
            println!("Found users, adding fiweeends!!{:?}", users);
            users
                .into_iter()
                .for_each(|user| match get_all_users(&connection) {
                    Ok(inner_users) => inner_users.into_iter().for_each(|inner_user| {
                        if user.id != inner_user.id {
                            println!("Adding user");
                            let friends = Friend {
                                user_id: user.id,
                                friend_id: inner_user.id,
                                added: date_now(),
                            };
                            println!("{:?}", add_friend_by_id(friends, &connection));
                        }
                    }),
                    _ => {}
                })
        }
        Err(err) => println!("Failed to get users: {:?}", err),
    }
}
