FROM rust:1.84-alpine

WORKDIR /usr/src/app

COPY . ./

RUN apk add musl-dev

RUN cargo build --release

CMD ["./target/release/vrbkapages-backend"]
