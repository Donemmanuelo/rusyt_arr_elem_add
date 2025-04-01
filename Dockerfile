FROM rust:latest As builder 

WORKDIR /app

COPY . .

RUN cargo build --release 

FROM debian:latest

RUN apt-get update  && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/add /app/add

CMD ["/app/add"]
