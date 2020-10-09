extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

fn main(){



    println!("Using bcrypt");
    let hashed = hash("hunter2", DEFAULT_COST).unwrap();
    let valid = verify("hunter2", &hashed).unwrap();

    println!("Bcrypt hash: {:?}", hashed);
    println!("Valid: {:?}", valid);



}