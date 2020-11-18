FROM rust:1.47-slim-buster as builder
WORKDIR /opt/app
COPY . /opt/app/
RUN apt update && apt install postgresql-server-dev-12 -y
#postgresql-client -y
# build-essential zlib1g-dev openssl -y
RUN cargo build --release && cp /opt/apt/gui /opt/app/target/release


FROM ubuntu:20.04 as runner
WORKDIR /opt/app
COPY --from=builder /opt/app/target/ /opt/app
CMD ./opt/app/target/release/main
EXPOSE 8080
