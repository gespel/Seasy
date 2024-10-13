FROM rust:latest
LABEL authors="stenh"

COPY . .

CMD cargo build

ENTRYPOINT ["./target/debug/seazy"]