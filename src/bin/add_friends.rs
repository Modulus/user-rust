use user_rust::db::lib::establish_connection;
use user_rust::db::users::create_user;
use user_rust::db::users::get_all_users;
use user_rust::db::friends::add_friend;
use user_rust::db::models::{NewUserJson, Friend, User};
use user_rust::errors::BackendError;

fn main(){
    let connection = establish_connection();

    match get_all_users(&connection) {
        Ok(users) =>  {
            println!("Found users, adding fiweeends!!{:?}", users);
            users.into_iter().for_each(| user| {
                match get_all_users(&connection){
                    Ok(inner_users) => {
                        inner_users.into_iter().for_each(| inner_user| {
                            if user.id != inner_user.id {
                                println!("Adding user");
                                let friends = Friend { user_id: user.id, friend_id: inner_user.id};
                                println!("{:?}", add_friend(friends, &connection));
                            }
                        })
                    }
                    _ => {}
                }
            })
        },
        Err(err) => println!("Failed to get users: {:?}", err)
    }

}