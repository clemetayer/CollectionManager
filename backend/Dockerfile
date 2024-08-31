FROM rust:1.78.0

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
ENV LOG_LEVEL=ERROR

RUN rm ./target/release/deps/backend*
RUN cargo install --path .

EXPOSE 8000

CMD ["sh", "-c", "RUST_LOG=${LOG_LEVEL} backend ${APP_ARGS}"]