FROM rust:1.51-buster
WORKDIR /opt/app

test:
    COPY . .
    ENV DATABASE_URL=postgres://user:user@localhost/user

    WITH DOCKER --compose docker-compose.yaml --allow-privileged
        RUN while ! pg_isready --host=localhost --port=5432 --dbname=iso3166 --username=shot; do sleep 1; done ;\
            cargo test
    END     

   
build:
    COPY . .
    RUN cargo test
    RUN cargo build --release && cp -r /opt/app/gui /opt/app/target/release
    SAVE ARTIFACT build/main /opt/app/target/release/main AS LOCAL build/main
docker:
    COPY +build/main .
    ENTRYPOINT ["/opt/app/target/release/main"]
    SAVE IMAGE user-rust:latest





