# Prerequisites
~~cargo install diesel_cli --features postgres,sqlite~~
cargo install diesel_cli --no-default-features --features "postgres sqlite"

# What
rust diesel and actix web app with vue 3 front for showing how user registration and login can be handled in rust


1. Create user diesel model
2. CRUD api for users
3. Login in actix
4. vue frontend

5. Jwt tokens: https://docs.rs/jsonwebtoken/7.2.0/jsonwebtoken/

# Create containers
docker-compose up -d

# Create migrations for postgres
diesel migrations run

# To clear database
diesel database reset

# To show users
cargo run --bin show_users

# To create hardcoded users
cargo run --bin create_users
cargo run --bin test_argon2

# Dump db
docker exec postgres pg_dumpall --user user < db/empty_db.sql

docker container exec -i $(docker-compose ps -q postgres) psql --user user < db/empty_db.sql

# Connection string
export DATABASE_URL=postgres://user:user@localhost/user

TODO: https://actix.rs/docs/databases/

## Earthly
https://earthly.dev/get-earthly

brew install earthly && sudo earthly bootstrap

See Earthfile for details
earthly --allow-privileged +test
earthly +build
earthly +docker

# Trigger build
