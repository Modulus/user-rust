
use argon2::{self, Config};

fn main(){



    let password = b"password";
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unrwap();
    let matches = argon2::verify_encoded(&hash, password).unwrap();
    assert!(matches);
    println!("Matches: {:?}", matches)


}