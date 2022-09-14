FROM rust as builder

# set working directory
WORKDIR /builder

# copy source code to /builder
COPY . .

# build binary with release profile
RUN cargo build --release --bin mena-rust

# move to alpine for final image
FROM debian:bookworm-slim as runtime

# set working directory
WORKDIR /mena-rust

# copy binary from builder
COPY --from=builder /build/target/release/mena-rust /usr/local/bin

# set it as an entrypoint
ENTRYPOINT ["./usr/local/bin/mena-rust"]