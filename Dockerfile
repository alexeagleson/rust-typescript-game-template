# Rust as the base image
FROM rust:1.66-slim-buster as builder

ENV project=gamejam

RUN apt update

RUN apt install -y pkg-config libssl-dev openssl

RUN cargo install sqlx-cli



WORKDIR /usr/src/${project}/ae-position

# Copy Cargo files
COPY ./ae-position/Cargo.toml .
COPY ./ae-position/Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

WORKDIR /usr/src/${project}/ae-direction

# Copy Cargo files
COPY ./ae-direction/Cargo.toml .
COPY ./ae-direction/Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

# FROM rust as rust-builder
WORKDIR /usr/src/${project}/

# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

RUN cargo build

RUN rm ./**/*.rs

COPY ./src ./src

COPY ./ae-direction/src ./ae-direction/src
COPY ./ae-position/src ./ae-position/src

COPY ./.env ./.env

COPY ./migrations ./migrations

RUN sqlx database create

RUN sqlx migrate run

RUN rm ./target/debug/deps/${project}*

RUN cargo build

EXPOSE 3030

# CLIENT


SHELL [ "/bin/bash", "-l", "-c" ]

RUN apt install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
# this now works
RUN nvm install 18

RUN nvm use 18

COPY ./client ./client

WORKDIR /usr/src/${project}/client

RUN npm install

RUN npm run build

RUN ls

WORKDIR /usr/src/${project}/

CMD [ "cargo", "run" ]


# for release

# # our final base
# FROM rust:1.66-slim-buster

# ENV project=gamejam

# # copy the build artifact from the build stage
# COPY --from=builder /usr/src/${project}/target/debug/${project} .

# WORKDIR /usr/src/${project}/

# EXPOSE 3030

# # needs to run the executable not cargo debug
# CMD [ "cargo", "run" ]