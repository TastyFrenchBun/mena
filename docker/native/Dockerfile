FROM rust as builder
WORKDIR build
COPY . .
RUN cargo build --release --bin mena-rust

FROM rust as runtime
WORKDIR build
COPY --from=builder /build/target/release/mena-rust /usr/local/bin
ENTRYPOINT ["./usr/local/bin/mena-rust"]