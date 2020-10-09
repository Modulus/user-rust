use sha2::{Digest, Sha512};
use std::str;



fn main(){

    let password = "1234password";
    let salt = "pepper";
    let mut hasher = Sha512::new();
    hasher.update(&password.as_bytes());
    hasher.update(b"$");
    hasher.update(salt.as_bytes());

    let hashed_pass = hasher.finalize();

    let hashed_hex = format!("{:x}", hashed_pass);

    println!("{:?}", hashed_hex);



}