use crate::db::models::{Friend, User};
use crate::db::users::get_user_by_id;
use crate::errors::BackendError;
use diesel::prelude::*;

pub fn add_fiend(user: &User, friend: &User, conn: &PgConnection) -> Result<usize, BackendError> {
    let new_friend = Friend {
        user_id: user.id,
        friend_id: friend.id,
        added: chrono::offset::Utc::now().naive_utc(),
    };

    return add_friend_by_id(new_friend, conn);
}

pub fn add_friend_by_id(friends: Friend, conn: &PgConnection) -> Result<usize, BackendError> {
    use crate::schema::friends;

    return Ok(diesel::insert_into(friends::table)
        .values(&friends)
        .execute(conn)?);
}

// pub fn get_friend_by_id(friend_id: i32, conn: &PgConnection) -> Result<User, BackendError> {}

pub fn list_friends(user: &User, conn: &PgConnection) -> Result<Vec<User>, BackendError> {
    use crate::schema::friends::dsl::*;

    let users_friends: Vec<Friend> = friends
        .filter(user_id.eq(user.id))
        .limit(25)
        .load::<Friend>(conn)
        .expect("Error loading fiwends!");

    println!("Listing friends: {:?}", users_friends);

    let mut friend_list: Vec<User> = Vec::new();

    for friend in users_friends {
        let user_from_id = get_user_by_id(conn, friend.friend_id)?;
        println!("Found friend: {:?}", user_from_id);
        friend_list.push(user_from_id);
    }

    return Ok(friend_list);
}

pub fn list_friends_by_id(id: i32, conn: &PgConnection) -> Result<Vec<User>, BackendError> {
    use crate::schema::friends::dsl::*;

    let users_friends: Vec<Friend> = friends
        .filter(user_id.eq(id))
        .limit(25)
        .load::<Friend>(conn)
        .expect("Error loading fiwends!");

    println!("Listing friends: {:?}", users_friends);

    let mut friend_list: Vec<User> = Vec::new();

    for friend in users_friends {
        let user_from_id = get_user_by_id(conn, friend.friend_id)?;
        println!("Found friend: {:?}", user_from_id);
        friend_list.push(user_from_id);
    }

    return Ok(friend_list);
}

//TODO: TEST
// pub fn list_friends_by_id_safe(
//     id: i32,
//     conn: &PgConnection,
// ) -> Result<Vec<UserSafe>, BackendError> {
//     use crate::schema::friends::dsl::*;
//
//     let users_friends: Vec<Friend> = friends
//         .filter(user_id.eq(id))
//         .limit(25)
//         .load::<Friend>(conn)
//         .expect("Error loading fiwends!");
//
//     println!("Listing friends: {:?}", users_friends);
//
//     let mut friend_list: Vec<UserSafe> = Vec::new();
//
//     for friend in users_friends {
//         let user_from_id = get_user_by_id_safe(conn, friend.friend_id)?;
//         println!("Found friend: {:?}", user_from_id);
//         friend_list.push(user_from_id);
//     }
//
//     return Ok(friend_list);
// }

pub fn remove_friend(
    owner: &User,
    friend: &User,
    conn: &PgConnection,
) -> Result<usize, BackendError> {
    use crate::schema::friends::dsl::*;

    return Ok(
        diesel::delete(friends.filter(user_id.eq(owner.id).and(friend_id.eq(friend.id))))
            .execute(conn)?,
    );
}

#[cfg(test)]
mod test {
    use crate::db::friends::{add_fiend, add_friend_by_id, list_friends, remove_friend};
    use crate::db::lib::establish_connection;
    use crate::db::models::NewUserJson;
    use crate::db::users::{create_user, delete_user_by_name};

    #[test]
    fn int_crd_friends() {
        let connection = establish_connection();
        let new_user = NewUserJson {
            name: "test-user-friends-1".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret".to_string(),
        };
        let result1 = create_user(&connection, &new_user);

        assert!(result1.is_ok());

        let new_user = NewUserJson {
            name: "test-user-friends-2".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret1234".to_string(),
        };
        let result2 = create_user(&connection, &new_user);
        assert!(result2.is_ok());

        let new_user = NewUserJson {
            name: "test-user-friends-3".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret".to_string(),
        };
        let result3 = create_user(&connection, &new_user);

        assert!(result3.is_ok());

        let new_user = NewUserJson {
            name: "test-user-friends-4".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret1234".to_string(),
        };
        let result4 = create_user(&connection, &new_user);
        assert!(result4.is_ok());

        let user1 = result1.unwrap();
        let user2 = result2.unwrap();
        let user3 = result3.unwrap();
        let user4 = result4.unwrap();

        add_fiend(&user1, &user2, &connection);
        add_fiend(&user1, &user3, &connection);
        add_fiend(&user1, &user4, &connection);
        assert_eq!(list_friends(&user1, &connection).unwrap().len(), 3);

        add_fiend(&user2, &user1, &connection);
        add_fiend(&user2, &user3, &connection);
        add_fiend(&user2, &user4, &connection);
        assert_eq!(list_friends(&user2, &connection).unwrap().len(), 3);

        add_fiend(&user3, &user1, &connection);
        add_fiend(&user3, &user2, &connection);
        add_fiend(&user3, &user4, &connection);
        assert_eq!(list_friends(&user3, &connection).unwrap().len(), 3);

        add_fiend(&user4, &user1, &connection);
        add_fiend(&user4, &user2, &connection);
        add_fiend(&user4, &user3, &connection);
        assert_eq!(list_friends(&user4, &connection).unwrap().len(), 3);

        remove_friend(&user1, &user2, &connection);
        assert_eq!(list_friends(&user1, &connection).unwrap().len(), 2);

        remove_friend(&user1, &user3, &connection);
        assert_eq!(list_friends(&user1, &connection).unwrap().len(), 1);

        remove_friend(&user1, &user4, &connection);
        assert_eq!(list_friends(&user1, &connection).unwrap().len(), 0);

        remove_friend(&user2, &user1, &connection);
        assert_eq!(list_friends(&user2, &connection).unwrap().len(), 2);

        remove_friend(&user2, &user3, &connection);
        assert_eq!(list_friends(&user2, &connection).unwrap().len(), 1);

        remove_friend(&user2, &user4, &connection);
        assert_eq!(list_friends(&user2, &connection).unwrap().len(), 0);

        remove_friend(&user3, &user1, &connection);
        assert_eq!(list_friends(&user3, &connection).unwrap().len(), 2);

        remove_friend(&user3, &user2, &connection);
        assert_eq!(list_friends(&user3, &connection).unwrap().len(), 1);

        remove_friend(&user3, &user4, &connection);
        assert_eq!(list_friends(&user3, &connection).unwrap().len(), 0);

        remove_friend(&user4, &user1, &connection);
        assert_eq!(list_friends(&user4, &connection).unwrap().len(), 2);

        remove_friend(&user4, &user2, &connection);
        assert_eq!(list_friends(&user4, &connection).unwrap().len(), 1);

        remove_friend(&user4, &user3, &connection);
        assert_eq!(list_friends(&user4, &connection).unwrap().len(), 0);

        delete_user_by_name(&connection, &user1.name);
        delete_user_by_name(&connection, &user2.name);
        delete_user_by_name(&connection, &user3.name);
        delete_user_by_name(&connection, &user4.name);
    }
}
