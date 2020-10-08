# Prerequisites
~~cargo install diesel_cli --features postgres,sqlite~~
cargo install diesel_cli --no-default-features --features "postgres sqlite"

# What
rust diesel and actix web app with vue 3 front for showing how user registration and login can be handled in rust


1. Create user diesel model
2. CRUD api for users
3. Login in actix
4. vue frontend