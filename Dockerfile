FROM rust:1.27.1

WORKDIR /usr/src/actix-server/
COPY . .

RUN cargo clean
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

EXPOSE 8080

CMD ./target/release/actix-server