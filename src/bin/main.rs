use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use user_rust::db::lib::establish_connection;
use user_rust::db::database::{create_user_raw, get_all_users};
use user_rust::db::models::{NewUserJson, UserJson};
use actix_web::web::Json;
use actix_files as fs;

// #[get("/")]
// async fn debug() -> impl Responder {
//     println!("Debug!!!");
//     format!("Hello wøøøøørking!!! {:?}", "Svada")
// }


#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

async fn add_user(new_user: Json<NewUserJson>) -> Result<String> {
    println!("Inserting new user");

    let connection = establish_connection();
    create_user_raw(&connection, &new_user.name, &new_user.comment, new_user.active, &new_user.password);
    Ok(format!("Welcome {:?}", new_user.name))
}

pub async fn get_users() -> Result<Json<Vec<UserJson>>>{
    println!("Listing all users");
    let connection = establish_connection();

    let raw_users = get_all_users(&connection);

    let json_users = raw_users.into_iter().map( | user | UserJson {
        id: user.id,
        name: user.name.to_string(),
        comment: user.comment,
        active: user.active,
        password: "*******".to_string()
    }).collect();

    return Ok(Json(json_users))


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

        HttpServer::new(|| {
            App::new()
                .service(fs::Files::new("/", "./gui/dist"))
                .service(web::resource("/users/add").route(web::post().to(add_user)))
                .service(web::resource("/users").route(web::get().to(get_users)))
                .service(index)
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await


}