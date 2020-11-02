use user_rust::db::lib::establish_connection;
use user_rust::db::users::create_user;
use user_rust::db::users::get_all_users;
use user_rust::db::friends::add_friend;
use user_rust::db::models::{NewUserJson, NewFriend};

fn main(){
    let connection = establish_connection();
    //
    // match get_all_users(&connection) {
    //     Ok(users) =>  {
    //         println!("{:?}", users);
    //         users.into_iter().for_each(| user| {
    //             users.into_iter().for_each(| inner_user| {
    //                 if user.id != inner_user.id {
    //                     let friends = NewFriend{ user_id: user.id, friend_id: inner_user.id};
    //                     add_friend(friends, &connection);
    //                 }
    //             })
    //         })
    //     },
    //     Err(err) => println!("Failed to get users")
    // }

    let friends1 = NewFriend{user_id: 1, friend_id: 2};
    add_friend(friends1, &connection);
    let friends2 = NewFriend{user_id: 1, friend_id: 3};
    add_friend(friends2, &connection);

    let friends3 = NewFriend{user_id: 2, friend_id: 1};
    add_friend(friends3, &connection);
    let friends4 = NewFriend{user_id: 2, friend_id: 3};
    add_friend(friends4, &connection);

    let friends5 = NewFriend{user_id: 3, friend_id: 1};
    add_friend(friends5, &connection);
    let friends6 = NewFriend{user_id: 3, friend_id: 2};
    add_friend(friends6, &connection);

}