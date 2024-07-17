# This cleanly clones and compiles the project before its ran.
FROM rust:latest

WORKDIR /app

RUN apt-get update && \
    apt-get install -y git && \
    apt-get clean

RUN git clone https://github.com/CloudVEX/url-short.git

WORKDIR /app/url-short

COPY .env .env

RUN cargo build --release

EXPOSE 7890

CMD ["./target/release/url-short"]
