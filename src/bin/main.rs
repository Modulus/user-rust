use actix_files as fs;
use actix_web::web::Json;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::header;
use actix_cors::Cors;
use actix_web_prom::PrometheusMetrics;
use futures::future;
use std::borrow::Borrow;
use std::collections::HashMap;
use user_rust::db::friends::{add_fiend, list_friends, list_friends_by_id};
use user_rust::db::lib::establish_connection;
use user_rust::db::messages::{list_all_messages, send_message};
use user_rust::db::models::{
    FriendJson, JwtToken, Message, NewMessage, NewUserJson, User, UserJson, UserLogin,
};
use user_rust::db::users::{create_user_raw, get_all_users, get_user_by_id, get_user_by_name};
use user_rust::errors::{BackendError, BackendErrorKind};

//TODO: Add jwt verification to all call
// #[get("/")]
// async fn debug() -> impl Responder {
//     println!("Debug!!!");
//     format!("Hello wøøøøørking!!! {:?}", "Svada")
// }

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

async fn login(user: Json<UserLogin>) -> Result<Json<String>, BackendError> {
    let connection = establish_connection();
    let raw_user = get_user_by_name(&connection, &(user).name)?;
    let expected_hash = raw_user.pass_hash;

    return match argon2::verify_encoded(&expected_hash, user.password.as_bytes()) {
        Ok(valid) => match valid {
            true => {
                let token = JwtToken::generate_token(&user);
                Ok(Json(token))
            }
            false => Err(BackendError {
                message: "Failed to login".to_string(),
                backend_error_kind: BackendErrorKind::LoginError(String::from("Failed to login!")),
            }),
        },
        Err(error) => Err(BackendError {
            message: "Fatal error during login".to_string(),
            backend_error_kind: BackendErrorKind::FatalError(String::from("Failed to login!")),
        }),
    };
}

async fn add_user(new_user: Json<NewUserJson>) -> Result<Json<UserJson>, BackendError> {
    println!("Inserting new user");

    let connection = establish_connection();
    let response = create_user_raw(
        &connection,
        &new_user.name,
        &new_user.comment,
        new_user.active,
        &new_user.password,
    )?;

    let json_user = UserJson {
        id: response.id,
        name: response.name,
        comment: response.comment,
        active: response.active,
        password: "**********".to_string(),
    };

    Ok(Json(json_user))
}

async fn add_friend_rest(friends: Json<FriendJson>) -> Result<Json<usize>, BackendError> {
    let connection = establish_connection();
    let user = get_user_by_id(&connection, friends.user_id)?;
    let friend_to_add = get_user_by_id(&connection, friends.friend_id)?;

    let result = add_fiend(&user, &friend_to_add, &connection)?;

    return Ok(Json(result));
}

async fn list_friends_rest(user: Json<UserJson>) -> Result<Json<Vec<UserJson>>, BackendError> {
    let connection = establish_connection();
    let friends = list_friends_by_id(user.id, &connection)?;

    let json_friends: Vec<UserJson> = friends
        .iter()
        .map(|raw_user| UserJson {
            id: raw_user.id,
            name: raw_user.name.to_string(),
            comment: None, //Figure this one out
            active: raw_user.active,
            password: String::default(),
        })
        .collect();

    return Ok(Json(json_friends));
}

pub async fn get_users() -> Result<Json<Vec<UserJson>>, BackendError> {
    println!("Listing all users");
    let connection = establish_connection();

    //TODO: MAKE get_all_users use ?
    match get_all_users(&connection) {
        Ok(result) => {
            let json_users = result
                .into_iter()
                .map(|user| UserJson {
                    id: user.id,
                    name: user.name.to_string(),
                    comment: user.comment,
                    active: user.active,
                    password: "*******".to_string(),
                })
                .collect();

            Ok(Json(json_users))
        }
        Err(error) => Err(error),
    }
}

pub async fn get_user_by_id_rest() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn delete_user_rest() -> impl Responder {
    format!("hello from delete user")
}

pub async fn send_message_rest(message: Json<NewMessage>) -> Result<Json<String>, BackendError> {
    let connection = establish_connection();

    let sender = get_user_by_id(&connection, message.sender_user_id)?;

    let receiver = get_user_by_id(&connection, message.receiver_user_id)?;

    let result = send_message(
        &sender,
        &receiver,
        message.header.to_string(),
        message.message.to_string(),
        &connection,
    )?;

    return Ok(Json("Sent".to_ascii_lowercase()));
}

//TODO: Create a messageJson type with updated user info and stripped away sender info
pub async fn list_messages_rest(user: Json<UserJson>) -> Result<Json<Vec<Message>>> {
    let connection = establish_connection();

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
    println!("Serving api on 0.0.0.0:8080");


    // println!("Serving metrics on 0.0.0.0:3000");
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut labels = HashMap::new();
    labels.insert("app".to_string(), "rust-user".to_string());
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), Some(labels));
    // env_logger::init();
    // let main_server = HttpServer::new(move || {
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .wrap(Logger::default())
            .wrap(Cors::default()
                .allowed_origin("*")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600))
            .route("/login", web::post().to(login))
            .route("/users/add", web::post().to(add_user))
            .route("/users", web::get().to(get_users))
            .route("/friends/add", web::post().to(add_friend_rest))
            .route("/friends", web::get().to(list_friends_rest))
            .route("/messages/post", web::post().to(send_message_rest))
            .route("/messages", web::get().to(list_messages_rest))
            .service(web::resource("/health").to(health))
            .service(fs::Files::new("/", "./gui/dist"))
    })
    .bind("0.0.0.0:8080")?
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
