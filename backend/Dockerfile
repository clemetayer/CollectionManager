FROM rust:latest

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY .env .env
VOLUME /backend/db

RUN rm ./target/release/deps/backend*
RUN cargo install --path .

EXPOSE 8000

CMD ["backend"]