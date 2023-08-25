FROM rust:latest

WORKDIR /build

COPY . .
RUN cargo build --release

CMD [ "target/release/backend" ]
