use actix_web::{get, web, App, HttpServer, Responder, Result, middleware::Logger};
use user_rust::db::lib::establish_connection;
use user_rust::db::database::{create_user_raw, get_all_users};
use user_rust::db::models::{NewUserJson, UserJson};
use actix_web::web::Json;
use actix_files as fs;
use user_rust::errors::BackendError;

// #[get("/")]
// async fn debug() -> impl Responder {
//     println!("Debug!!!");
//     format!("Hello wøøøøørking!!! {:?}", "Svada")
// }


#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

async fn add_user(new_user: Json<NewUserJson>) -> Result<Json<UserJson>, BackendError> {
    println!("Inserting new user");

    let connection = establish_connection();
    let response = create_user_raw(&connection, &new_user.name, &new_user.comment, new_user.active, &new_user.password)?;
    
    let json_user = UserJson{
        id: response.id,
        name: response.name,
        comment: response.comment,
        active: response.active,
        password: "**********".to_string()
    };

    Ok(Json(json_user))
}

pub async fn get_users() -> Result<Json<Vec<UserJson>>, BackendError>{
    println!("Listing all users");
    let connection = establish_connection();

    //TODO: MAKE get_all_users use ?
    match get_all_users(&connection) {
        Ok(result) => {
            let json_users = result.into_iter().map( | user | UserJson {
                id: user.id,
                name: user.name.to_string(),
                comment: user.comment,
                active: user.active,
                password: "*******".to_string()
            }).collect();

            Ok(Json(json_users))
        },
        Err(error) => {
           Err(error)
        }
    }





}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}


pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Serving on 0.0.0.0:8080");
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
        HttpServer::new(|| {
            App::new().wrap(Logger::default())
                // .service(fs::Files::new("/", "./gui/dist"))
                .route("/users/add", web::post().to(add_user))
                .route("/users", web::get().to(get_users))
                .service(fs::Files::new("/", "./gui/dist"))
                // .service(web::resource("/users/add").route(web::post().to(add_user)))
                // .service(web::resource("/users").route(web::get().to(get_users)))
                // .service(index)
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await


}