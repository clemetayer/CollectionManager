FROM rust:latest

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY .env .env
COPY integration_tests.env integration_tests.env
VOLUME /backend/db

ARG app_args
ENV APP_ARGS=$app_args 

RUN rm ./target/release/deps/backend*
RUN cargo install --path .

EXPOSE 8000

CMD ["sh", "-c", "backend ${APP_ARGS}"]