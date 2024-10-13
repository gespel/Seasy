FROM rust:latest
LABEL authors="stenh"

EXPOSE 3000

COPY . .

RUN cargo build

ENTRYPOINT ["./target/debug/seazy"]