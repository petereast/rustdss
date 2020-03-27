FROM rust:1.40 as builder
WORKDIR /usr/src/rustdss
COPY . .
RUN cd rustdss_server && cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/rustdss /usr/local/bin/rustdss_server
EXPOSE 6379
CMD ["rustdss_server"]
