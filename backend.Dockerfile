FROM rust:1.84-alpine

WORKDIR /usr/src/app/


RUN apk add musl-dev

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

COPY frontend ./

COPY backend ./

WORKDIR /usr/src/app/frontend

RUN trunk build --release

RUN mv dist ../backend/static

WORKDIR /usr/src/app/backend

RUN cargo build --release

CMD ["./target/release/vrbkapages-backend"]
