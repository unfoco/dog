FROM rust:1.81.0 as build

RUN apk update
RUN apk add pkgconfig openssl openssl-dev musl-dev

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup toolchain install stable-x86_64-unknown-linux-musl

WORKDIR /src

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.20.3

WORKDIR /app

COPY --from=build /src/target/x86_64-unknown-linux-musl/release/dog .

CMD ["./dog"]
