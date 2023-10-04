FROM rust:1.72.1 as build
WORKDIR /usr/src/viewgame

COPY src/ src/
COPY Cargo.toml .
COPY .env .

RUN cargo install --path .


CMD ["viewgame"]