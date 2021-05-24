FROM rust:1.52 as build
COPY . /app/
WORKDIR /app/
RUN cargo build --release

FROM debian:bullseye
COPY --from=build /app/target/release/tinytrix /usr/local/bin/tinytrix
ENTRYPOINT /usr/local/bin/tinytrix
