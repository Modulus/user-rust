use actix_cors::Cors;
use actix_files as fs;
use actix_web::web::Json;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_prom::PrometheusMetrics;
use env_logger::Env;

use log::{info, warn};

use std::collections::HashMap;
use user_rust::db::friends::{add_fiend, list_friends_by_id};
use user_rust::db::lib::establish_connection;
use user_rust::db::messages::{list_all_messages, send_message};
use user_rust::db::models::{
    FriendJson, TokenHelper, Message, NewMessage, NewUserJson, UserJson, UserLogin,
};
use user_rust::db::users::{create_user_raw, get_all_users, get_user_by_id, get_user_by_name};
use user_rust::errors::{BackendError, BackendErrorKind};

// TODO: https://turreta.com/2020/06/07/actix-web-basic-and-bearer-authentication-examples/
//TODO: Add jwt verification to all call
// #[get("/")]
// async fn debug() -> impl Responder {
//     println!("Debug!!!");
//     format!("Hello wøøøøørking!!! {:?}", "Svada")
// }

// async fn bearer_auth_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.get_ref().clone())
//         .unwrap_or_else(Default::default);
//     match validate_token(credentials.token()) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

async fn login(user: Json<UserLogin>) -> Result<Json<String>, BackendError> {
    let connection = establish_connection();
    let raw_user = get_user_by_name(&connection, &(user).name)?;
    let expected_hash = raw_user.pass_hash;

    info!("Checking user!");

    return match argon2::verify_encoded(&expected_hash, user.password.as_bytes()) {
        Ok(valid) => match valid {
            true => {
                let token = TokenHelper::generate_token(&user);
                info!("Password matched hash, returning JWT token!");
                Ok(Json(token))
            }
            false => {
                warn!("No matching user!");
                Err(BackendError {
                    message: "Failed to login".to_string(),
                    backend_error_kind: BackendErrorKind::LoginError
                })
            }
        },
        Err(error) => Err(BackendError {
            message: format!("Fatal error during login, {:?}", error),
            backend_error_kind: BackendErrorKind::FatalError,
        }),
    };
}

async fn add_user(new_user: Json<NewUserJson>) -> Result<Json<UserJson>, BackendError> {
    info!("Inserting new user");

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
    info!("Listing all users");
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

    println!("{:?}", result);

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
    println!("Serving api on 0.0.0.0:8888");

    // println!("Serving metrics on 0.0.0.0:3000");
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "full");
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("INFO")).init();

    // let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    // let pool = Pool::builder().build(manager).expect("Failed to create pool");
    // let repo = UserRepository{
    //     pool: pool
    // };



    // env_logger::init();
    // let main_server = HttpServer::new(move || {

    HttpServer::new(move || {
        // let auth = HttpAuthentication::basic(basic_auth_validator);
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "rust-user".to_string());        
        let prometheus = PrometheusMetrics::new("api", Some("/metrics"), Some(labels));

        App::new()
            // .wrap(
            //     Cors::default().allowed_origin("*"), // .allowed_methods(vec!["GET", "POST"])
            //                                          // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //                                          // .allowed_header(header::CONTENT_TYPE)
            //                                          // .max_age(3600)
            // )
            // .wrap(auth)
            .wrap(Cors::permissive())
            .wrap(prometheus)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
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
