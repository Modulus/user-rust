use actix_cors::Cors;
use actix_files as fs;
use actix_web::{HttpRequest, web::Json};
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_prom::PrometheusMetrics;
use diesel::{PgConnection, r2d2::ConnectionManager, r2d2::{self, Pool}};
use env_logger::Env;
use dotenv::dotenv;
use log::{debug, error, info, warn};
use web::resource;

use std::{borrow::Borrow, collections::HashMap, env};
use user_rust::{db::{friends::{add_fiend, list_friends_by_id}, models::{User, generate_token}, users::UserRepository}};
use user_rust::db::messages::{list_all_messages, send_message};
use user_rust::db::models::{
    FriendJson, TokenHelper, Message, NewMessage, NewUserJson, UserLogin,
};
use user_rust::db::users::{create_user_raw, get_user_by_id, get_user_by_name};
use user_rust::errors::{BackendError, BackendErrorKind};


// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

async fn login(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, user: Json<UserLogin>) -> Result<Json<String>, BackendError> {
    let connection = pool.get().unwrap(); //TODO: Fix error handling
    let raw_user = get_user_by_name(&connection, &(user).name)?;
    let expected_hash = raw_user.pass_hash;

    info!("Checking user!");

    return match argon2::verify_encoded(&expected_hash, user.password.as_bytes()) {
        Ok(valid) => match valid {
            true => {
                let token = generate_token(&user)?;
                info!("Password matched hash, returning JWT token!");
                Ok(Json(token))
            }
            false => {
                warn!("No matching user!");
                Err(BackendError {
                    message: "Failed to login".to_string(),
                    kind: BackendErrorKind::LoginError
                })
            }
        },
        Err(error) => Err(BackendError {
            message: format!("Fatal error during login, {:?}", error),
            kind: BackendErrorKind::FatalError,
        }),
    };
}

async fn add_user(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, new_user: Json<NewUserJson>) -> Result<Json<User>, BackendError> {
    info!("Inserting new user");

    let connection = pool.get().unwrap(); //TODO: Add better error handling
    let response = create_user_raw(
        &connection,
        &new_user.name,
        &new_user.comment,
        new_user.active,
        &new_user.password,
    )?;


    Ok(Json(response))
}

async fn add_friend_rest(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, friends: Json<FriendJson>) -> Result<Json<usize>, BackendError> {
    let connection = pool.get().unwrap(); //TODO: Add better error handling
    let user = get_user_by_id(&connection, friends.user_id)?;
    let friend_to_add = get_user_by_id(&connection, friends.friend_id)?;

    let result = add_fiend(&user, &friend_to_add, &connection)?;

    return Ok(Json(result));
}

async fn list_friends_rest(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, user: Json<User>) -> Result<Json<Vec<User>>, BackendError> {

    match pool.get(){
        Ok(connection) => {
            let friends = list_friends_by_id(user.id, &connection)?;
            return Ok(Json(friends));

        }
        Err(error) => {
            Err(BackendError{ message: error.to_string(), kind: BackendErrorKind::DieselError})
        }
    }



    // let json_friends: Vec<UserJson> = friends
    //     .iter()
    //     .map(|raw_user| UserJson {
    //         id: raw_user.id,
    //         name: raw_user.name.to_string(),
    //         comment: None, //Figure this one out
    //         active: raw_user.active,
    //         password: String::default(),
    //     })
    //     .collect();

}

pub async fn get_users(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, _token_session: TokenHelper) -> Result<Json<Vec<User>>, BackendError> {
    info!("Listing all users");
    debug!("Access granted!");
    let repo = UserRepository{pool: pool.get_ref()};

    return Ok(Json(repo.get_all_users(25)?));
}

pub async fn get_user_by_id_rest() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn delete_user_rest(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, web::Path((name)): web::Path<(String)>, _token_session: TokenHelper) -> Result<Json<usize>, BackendError> { //_token_session: TokenHelper
    

    warn!("Deleting user!");
     info!("Deleting user with name: {}", name);
    let repo = UserRepository{pool: pool.get_ref()};
    // warn!("Fetching user!");
    match repo.get(name.to_string().borrow()){
        Ok(_result) => {
            info!("Found matching user! Deleting");
            return Ok(Json(repo.delete(&name)?));

        }
        Err(error) => {
            error!("Something failed: {:?}", error);
            return Err(BackendError{ message: "Failed to delete user!".to_string(), kind: BackendErrorKind::FatalError});
        }
    };


}

pub async fn send_message_rest(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, message: Json<NewMessage>) -> Result<Json<String>, BackendError> {
    let connection = pool.get().unwrap(); //TODO: Add better error handling

    let sender = get_user_by_id(&connection, message.sender_user_id)?;

    let receiver = get_user_by_id(&connection, message.receiver_user_id)?;

    let result = send_message(
        &sender,
        &receiver,
        message.header.to_string(),
        message.message.to_string(),
        &connection,
    )?;

    println!("{:?}", result);

    return Ok(Json("Sent".to_ascii_lowercase()));
}

//TODO: Create a messageJson type with updated user info and stripped away sender info
pub async fn list_messages_rest(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, user: Json<User>) -> Result<Json<Vec<Message>>> {
    let connection = pool.get().unwrap(); //TODO: Add better error handling

    let user = get_user_by_id(&connection, user.id)?;

    return Ok(Json(list_all_messages(&user, &connection)?));
}

pub async fn debug() -> Result<String> {
    return Ok(String::from("Debug"));
}

fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving api on 0.0.0.0:8888");

    // println!("Serving metrics on 0.0.0.0:3000");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "full");
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();


    HttpServer::new(move || {
        // let auth = HttpAuthentication::basic(basic_auth_validator);
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "rust-user".to_string());        
        let prometheus = PrometheusMetrics::new("api", Some("/metrics"), Some(labels));
        //Find a way to trigger this in a good way
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(1).build(manager).expect("Failed to create pool");

             // .wrap(
            //     Cors::default().allowed_origin("*"), // .allowed_methods(vec!["GET", "POST"])
            //                                          // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //                                          // .allowed_header(header::CONTENT_TYPE)
            //                                          // .max_age(3600)
            // )
            // .wrap(auth)

        App::new().data(pool)
            .wrap(Cors::permissive())
            .wrap(prometheus)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/login", web::post().to(login))
            .route("/users/add", web::post().to(add_user))
            .service(resource("/users/delete/{id}/{name}").route(web::delete().to(delete_user_rest)))
            .route("/users", web::get().to(get_users))
            .route("/friends/add", web::post().to(add_friend_rest))
            .route("/friends", web::get().to(list_friends_rest))
            .route("/messages/post", web::post().to(send_message_rest))
            .route("/messages", web::get().to(list_messages_rest))
            .service(web::resource("/health").to(health))
            .service(fs::Files::new("/", "./gui/dist"))
    })
    .bind("0.0.0.0:8888")?
    .run()
    .await?;

    Ok(())

    // let metrics_server = HttpServer::new(move || {
    //     App::new()
    //         .wrap(prometheus.clone())
    //         .service(web::resource("/health").to(health))
    // })
    // .bind("0.0.0.0:3000")?
    // .run();
    //
    // future::try_join(main_server, metrics_server).await?;
    //
    // Ok(())
}


#[cfg(test)]
mod tests {
    use std::io::Bytes;

    use super::*;
    use actix_http::{body::Body, http::Method};
    use actix_web::{App, test, web::{self, json}};
    use web::service;

    #[actix_rt::test]
    async fn test_login_with_no_credentials_throws_server_error() {
        let mut app = test::init_service(App::new().route("/", web::get().to(login))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_server_error());
    }

    #[actix_rt::test]
    async fn test_login_has_credentials_but_not_registered_user_fails() {
        let mut app = test::init_service(App::new().route("/", web::get().to(login))).await;

        let login = UserLogin{ name: "nobody".to_string(), password: "My secret".to_string()};
        let req = test::TestRequest::with_header("content-type", "text/plain").set_json(&login).to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_server_error());
    }

    #[actix_rt::test]
    async fn test_login_after_created_user_login_is_success_full(){

        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(1).build(manager).expect("Failed to create pool");


        let mut app = test::init_service(App::new()
            .data(pool)
            .route("/login", web::get().to(login))
            .route("/register", web::post().to(add_user))
            .service(resource("/delete/{name}").route(web::delete().to(delete_user_rest))))
            .await;


        let new_user = NewUserJson{ name: "nobody".to_string(), comment: "Empty".to_string(), active: true, password: "My secret 1234".to_string()};

        let register_req = test::TestRequest::with_header("content-type", "application/json").set_json(&new_user).method(Method::POST).uri("/register").to_request();
        let register_resp = test::call_service(&mut app, register_req).await;
        assert!(register_resp.status().is_success());


        let login = UserLogin{ name: "nobody".to_string(), password: "My secret 1234".to_string()};
        let req = test::TestRequest::with_header("content-type", "application/json").set_json(&login).method(Method::GET).uri("/login").to_request();
        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = resp.take_body().as_ref().unwrap();

        let body_string = String::from_utf8(&body.unwrap();
        

        // let delete_req = test::TestRequest::with_header("Authorization", format!("Bearer \"{}\"", body).set_payload("nobody").method(Method::DELETE).uri("/delete").to_request());
        // let delete_resp = test::call_service(&mut app, req).await;

        // assert!(delete_resp.status().is_success());

        println!("NOOOO!");
    }

    #[actix_rt::test]
    async fn test_double_registered_user_fails_second_time(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(1).build(manager).expect("Failed to create pool");


        let mut app = test::init_service(App::new()
            .data(pool)
            .route("/login", web::get().to(login))
            .route("/register", web::post().to(add_user))
            .route("/delete", web::delete().to(delete_user_rest)))
            .await;


        let new_user = NewUserJson{ name: "nobody-second".to_string(), comment: "Empty".to_string(), active: true, password: "My secret 1234".to_string()};

        let register_req1 = test::TestRequest::with_header("content-type", "application/json").set_json(&new_user).method(Method::POST).uri("/register").to_request();
        let register_resp1 = test::call_service(&mut app, register_req1).await;
        assert!(register_resp1.status().is_success());


        let register_req2 = test::TestRequest::with_header("content-type", "application/json").set_json(&new_user).method(Method::POST).uri("/register").to_request();
        let register_resp2 = test::call_service(&mut app, register_req2).await;
        assert!(register_resp2.status().is_server_error());

    }
 
}
