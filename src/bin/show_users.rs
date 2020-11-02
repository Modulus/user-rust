use user_rust::db::lib::establish_connection;
use user_rust::db::users::show_users;

fn main(){
    let connection = establish_connection();
    show_users(&connection);

}