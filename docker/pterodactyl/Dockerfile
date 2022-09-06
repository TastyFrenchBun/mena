# get rust image
FROM rust AS builder

# set working directory
WORKDIR /build

# copy source code
COPY . .

# build binary with release profile
RUN cargo build --release --bin mena-rust

# get alpine for final image
FROM alpine:latest

# create user for pterodactyl
RUN adduser --disabled-password --home /home/container container

# set working directory
WORKDIR /home/container
ENV USER container
ENV HOME /home/container

# copy binary from builder
COPY --from=builder /build/target/release/mena-rust .

# set user to container
USER container

COPY --from=builder /build/docker/pterodactyl/start.sh /start.sh
COPY --from=builder /build/docker/pterodactyl/entrypoint.sh /entrypoint.sh
CMD ["/bin/ash", "/entrypoint.sh"]