use actix_cors::Cors;
use actix_files as fs;
use actix_http::HttpMessage;
use actix_web::{HttpRequest, web::Json};
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_prom::PrometheusMetrics;
use diesel::{PgConnection, r2d2::ConnectionManager, r2d2::{self, Pool}};
use env_logger::Env;
use dotenv::dotenv;
use log::{debug,info, warn};

use std::{borrow::Borrow, collections::HashMap, env};
use user_rust::db::{friends::{add_fiend, list_friends_by_id}, models::User, users::UserRepository};
use user_rust::db::lib::establish_connection;
use user_rust::db::messages::{list_all_messages, send_message};
use user_rust::db::models::{
    FriendJson, TokenHelper, Message, NewMessage, NewUserJson, UserLogin,
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

async fn login(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, user: Json<UserLogin>) -> Result<Json<String>, BackendError> {
    let connection = pool.get().unwrap(); //TODO: Fix error handling
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
    let connection = pool.get().unwrap(); //TODO: ADD better error handling
    let friends = list_friends_by_id(user.id, &connection)?;

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

    return Ok(Json(friends));
}

pub async fn get_users(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>, request: HttpRequest) -> Result<Json<Vec<User>>, BackendError> {
    info!("Listing all users");
    info!("{:#?}", request);

    let repo = UserRepository{pool: pool.get_ref()};


    //TODO: Create method for this
    match request.headers().get("authorization"){
        Some(header) => {
            debug!("Found token");

            let token = TokenHelper::extract_token_from_header_value(header.to_str()?).expect("Failed to to get header for auth token!");
            if TokenHelper::validate_token(token.borrow()){
                return Ok(Json(repo.get_all_users(25)?));
            }

            return Err(BackendError{
                message: "Invalid token".to_string(),
                backend_error_kind: BackendErrorKind::AuthError,

            });

        },
        None => {
            Err(BackendError{
                message: "You do not have access to this!".to_string(),
                backend_error_kind: BackendErrorKind::AuthError,
                
            })
        }
    }

}

pub async fn get_user_by_id_rest() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn delete_user_rest() -> impl Responder {
    format!("hello from delete user")
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


    // let repo = UserRepository{
    //     pool: pool
    // };
    // let manager = ConnectionManager::<PgConnection>::new("");
    // let pool = Pool::builder().build(manager).expect("Failed to create pool");
    // HttpServer::new(move || {
    //     App::new(pool.clone())
    //         .resource("/", web::get().to(login))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await;

    // env_logger::init();
    // let main_server = HttpServer::new(move || {

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
