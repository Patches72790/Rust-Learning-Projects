FROM rust

WORKDIR /build
COPY . .

RUN cargo build --release

EXPOSE 3000

CMD [ "/build/target/release/axum-project" ]
