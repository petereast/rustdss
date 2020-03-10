FROM rust:1.40 as builder
WORKDIR /usr/src/rustdss
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rustdss /usr/local/bin/rustdss
CMD ["rustdss"]
