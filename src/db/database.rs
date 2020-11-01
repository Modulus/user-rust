use std::str;

use diesel::prelude::*;
use crate::db::models::{User, NewUser, NewUserJson};
use argon2::{self, Config};
use rand::Rng; 
use rand::distributions::Alphanumeric;

pub fn create_user(conn: &PgConnection, user: NewUserJson) -> User {
    use crate::schema::users;

    let salt_length : usize = 30;
    let salt = create_salt(salt_length);

    let hashed_pass = create_hash(&user.password, &salt);

    let new_user = NewUser{
        name: &user.name,
        comment: &user.comment,
        active: user.active,
        pass_hash: &hashed_pass
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user!")
}
pub fn create_user_raw<'a>(conn: &PgConnection, name: &'a str, comment: &'a str, active: bool, password: &'a str) -> User {
    use crate::schema::users;

    let salt_length : usize = 30;
    let salt = create_salt(salt_length);

    let hashed_pass = create_hash("my password", &salt);

    let new_user = NewUser{
        name: name,
        comment: comment,
        active: active,
        pass_hash: &hashed_pass
    };


    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user!")

}

pub fn create_hash(password: &str, salt: &str) -> ResulsString {
    let config = Config::default();
    let hash = argon2::hash_encoded(&password.as_bytes(), &salt.as_bytes(), &config)?;
    return hash
}

pub fn create_salt(length: usize) -> String {
    return rand::thread_rng().sample_iter(&Alphanumeric).take(length).collect::<String>();
}

pub fn show_users(conn: &PgConnection) {
    use crate::schema::users::dsl::*;
    let result = users.filter(active.eq(true))
        .limit(10)
        .load::<User>(conn)
        .expect("Error loading users!");

    println!("Displaying {}", result.len());
    for user in result {
        println!("User: {:?}", user.name);
        println!("Hash: {:?}", user.pass_hash);
        println!("Comment: {:?}", user.comment);
    }

}

pub fn get_user_by_name(conn: &PgConnection, name: &str) -> Result<Vec<User>, String>{
    use crate::schema::users::dsl::*;

    return users.filter(name.eq(name)).limit(1).load::<User>(conn)?;


}

pub fn get_all_users(conn: &PgConnection) -> Vec<User>{
    use crate::schema::users::dsl::*;
    let result = users.filter(active.eq(true))
        .limit(10)
        .load::<User>(conn).unwrap();

    return result;
}