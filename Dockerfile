FROM rust:latest
LABEL authors="stenh"

EXPOSE 3000

COPY . .


ENTRYPOINT ["cargo", "run"]