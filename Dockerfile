FROM rust:latest As builder 

WORKDIR /app

COPY . .

RUN cargo build --release 

FROM debian:alpine 

RUN apt-get update  && apt-get install -y libssl-dev

COPY --from=builder /app/target/src/release/add /app/add

CMD ["/app/add"]