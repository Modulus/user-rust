use user_rust::db::lib::establish_connection;
use user_rust::db::database::show_users;

fn main(){
    let connection = establish_connection();
    show_users(&connection);

}