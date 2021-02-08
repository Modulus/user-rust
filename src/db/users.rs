use std::str;

use crate::db::models::{NewUser, NewUserJson, User};
use crate::errors::BackendError;
use argon2::{self, Config};
use diesel::prelude::*;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError , Error};
use rand::distributions::Alphanumeric;
use rand::Rng;


pub fn create_hash(password: &str, salt: &str) -> Result<String, BackendError> {
    let config = Config::default();
    Ok(argon2::hash_encoded(
        &password.as_bytes(),
        &salt.as_bytes(),
        &config,
    )?)
}

pub fn create_salt(length: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect::<String>();
}


pub struct UserRepository {
    pool: Pool<ConnectionManager<diesel::PgConnection>>
}

impl UserRepository {
    // pub fn new(self, pool: Pool<ConnectionManager<diesel::PgConnection>>){
    //     self.pool = pool;
    // }

    pub fn create(&self, user: &NewUserJson) -> Result<User, BackendError> {
        use crate::schema::users;

        let conn = self.pool.get().unwrap();

        let salt_length: usize = 30;
        let salt = create_salt(salt_length);
    
        let hashed_pass = create_hash(&user.password, &salt);
    
        let new_user = NewUser {
            name: &user.name,
            comment: &user.comment,
            active: user.active,
            pass_hash: &hashed_pass?,
            created: chrono::offset::Utc::now().naive_utc(),
        };
    
        let result = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&conn)?;
    
        return Ok(result);        
    }

    pub fn delete(&self, _name: &str) -> Result<usize, BackendError> {
        use crate::schema::users::dsl::*;
    
        let conn = self.pool.get().unwrap();
        return Ok(diesel::delete(users.filter(name.eq(_name))).execute(&conn)?);
    }

    pub fn get(&self, _name: &str) -> Result<User, BackendError> {
        use crate::schema::users::dsl::*;
        let conn = self.pool.get().unwrap();
        return Ok(users
            .filter(name.eq(_name))
            // .select((id, name, comment, active, created))
            .first::<User>(&conn)?);
    }

    pub fn get_all_users(&self, limit: i64) -> Result<Vec<User>, BackendError> {
        use crate::schema::users::dsl::*;
        let conn = self.pool.get().unwrap();
        Ok(users
            .filter(active.eq(true))
            // .select((id, name, comment, active, created))
            .limit(limit)
            .load::<User>(&conn)?)
    }

    pub fn get_user_by_id(&self, _id: i32) -> Result<User, BackendError> {
        use crate::schema::users::dsl::*;
        let conn = self.pool.get().unwrap();        
        Ok(users
            .filter(id.eq(_id))
            // .select((id, name, comment, active, created))
            .first::<User>(&conn)?)
    }

    pub fn create_user_raw<'a>( &self,
        name: &'a str,
        comment: &'a str,
        active: bool,
        password: &'a str,
    ) -> Result<User, BackendError> {
        use crate::schema::users;
    
        let salt_length: usize = 30;
        let salt = create_salt(salt_length);
    
        let hashed_pass = create_hash(password, &salt)?;
    
        let new_user = NewUser {
            name: name,
            comment: comment,
            active: active,
            pass_hash: &*hashed_pass,
            created: chrono::offset::Utc::now().naive_utc(),
        };

        let conn = self.pool.get().unwrap();
    
        Ok(diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&conn)?)
    }
    
}

pub fn create_user(conn: &PgConnection, user: &NewUserJson) -> Result<User, BackendError> {
    use crate::schema::users;

    let salt_length: usize = 30;
    let salt = create_salt(salt_length);

    let hashed_pass = create_hash(&user.password, &salt);

    let new_user = NewUser {
        name: &user.name,
        comment: &user.comment,
        active: user.active,
        pass_hash: &hashed_pass?,
        created: chrono::offset::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?;

    return Ok(result);
}
pub fn create_user_raw<'a>(
    conn: &PgConnection,
    name: &'a str,
    comment: &'a str,
    active: bool,
    password: &'a str,
) -> Result<User, BackendError> {
    use crate::schema::users;

    let salt_length: usize = 30;
    let salt = create_salt(salt_length);

    let hashed_pass = create_hash(password, &salt)?;

    let new_user = NewUser {
        name: name,
        comment: comment,
        active: active,
        pass_hash: &*hashed_pass,
        created: chrono::offset::Utc::now().naive_utc(),
    };

    Ok(diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)?)
}


pub fn show_users(conn: &PgConnection) {
    use crate::schema::users::dsl::*;
    let result = users
        .filter(active.eq(true))
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

pub fn get_user_by_id(conn: &PgConnection, _id: i32) -> Result<User, BackendError> {
    use crate::schema::users::dsl::*;
    Ok(users
        .filter(id.eq(_id))
        // .select((id, name, comment, active, created))
        .first::<User>(conn)?)
}

// pub fn get_user_by_id_safe(conn: &PgConnection, _id: i32) -> Result<UserSafe, BackendError> {
//     use crate::schema::users::dsl::*;
//     Ok(users
//         .filter(id.eq(_id))
//         .select((id, name, comment, created))
//         .first::<UserSafe>(conn)?)
// }

pub fn get_user_by_name(conn: &PgConnection, _name: &str) -> Result<User, BackendError> {
    use crate::schema::users::dsl::*;

    return Ok(users
        .filter(name.eq(_name))
        // .select((id, name, comment, active, created))
        .first::<User>(conn)?);
}

pub fn delete_user_by_name(conn: &PgConnection, _name: &str) -> Result<usize, BackendError> {
    use crate::schema::users::dsl::*;

    return Ok(diesel::delete(users.filter(name.eq(_name))).execute(conn)?);
}

pub fn get_all_users(conn: &PgConnection) -> Result<Vec<User>, BackendError> {
    use crate::schema::users::dsl::*;
    Ok(users
        .filter(active.eq(true))
        // .select((id, name, comment, active, created))
        .limit(25)
        .load::<User>(conn)?)
}

// // pub fn get_all_users_safe(conn: &PgConnection) -> Result<Vec<UserSafe>, BackendError> {
// //     use crate::schema::users::dsl::*;
// //     Ok(users
// //         .filter(active.eq(true))
// //         .select((id, name, comment, created))
// //         .limit(25)
// //         .load::<UserSafe>(conn)?)
// // }

#[cfg(test)]
mod tests {
    use crate::db::lib::establish_connection;
    use crate::db::models::{NewUserJson};
    use crate::db::users::UserRepository;
    use crate::db::users::{create_user, delete_user_by_name, get_user_by_id, get_user_by_name};
    use diesel::{pg::PgConnection, r2d2::ConnectionManager, r2d2::Pool};
    #[test]
    fn it_crud_repo(){
        let DATABASE_URL = "postgres://user:user@localhost/user".to_string();

        let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");
        let repo = UserRepository{
            pool: pool
        };

        let new_user = NewUserJson {
            name: "test-user1".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret".to_string(),
        };
        let result =         repo.create(&new_user);

        assert!(result.is_ok());

        let result_delete = repo.delete(&new_user.name);

        assert!(result_delete.unwrap() > 0);

        let user = repo.get(&new_user.name);
        assert!(user.is_err());



    }

    // #[test]
    // fn it_create_user_works() {
    //     let conn = establish_connection();
    //     let new_user = NewUserJson {
    //         name: "test-user1".to_string(),
    //         comment: "User created from intagration test".to_string(),
    //         active: true,
    //         password: "supersecret".to_string(),
    //     };
    //     let result = create_user(&conn, &new_user);

    //     assert!(result.is_ok());
    //     let user = result.unwrap();

    //     let user_by_name = get_user_by_name(&conn, &new_user.name).unwrap();

    //     let user_by_id = get_user_by_id(&conn, user_by_name.id).unwrap();
    //     assert_eq!(&user, &user_by_name);
    //     assert_eq!(&user_by_name, &user_by_id);

    //     delete_user_by_name(&conn, &new_user.name);

    //     let existing_user = get_user_by_name(&conn, &new_user.name);
    //     assert!(existing_user.is_err());
    // }
}
