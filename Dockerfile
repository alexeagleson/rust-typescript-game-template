FROM rust:1.66-alpine

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["myapp"]



https://dev.to/rogertorres/first-steps-with-docker-rust-30oi