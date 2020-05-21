#FROM rust as builder
# Alpine uses a patched rust that can link musl dynamically
# which is needed for procedural macros. The above line using the
# official rust image can be enabled once rust 1.44 is released.
FROM rust:slim as builder

#RUN mkdir -p /usr/src
WORKDIR /usr/src/
RUN apt-get update && apt-get install -y libssl-dev pkg-config
#RUN apk add rust cargo openssl-dev

COPY Cargo.toml Cargo.lock /usr/src
COPY tavern_server /usr/src/tavern_server
COPY tavern_pathfinder /usr/src/tavern_pathfinder
COPY tavern_derive /usr/src/tavern_derive
RUN cargo install --path /usr/src/tavern_server/

FROM ubuntu:latest
RUN apt-get update && apt-get install -y libssl1.1
COPY --from=builder /usr/local/cargo/bin/tavern_server /usr/local/bin/tavern
CMD ["tavern"]
