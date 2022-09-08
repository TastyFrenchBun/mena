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

# set user to container
# will uncomment after release.....
# USER container

# set working directory
WORKDIR /home/container
ENV USER container
ENV HOME /home/container

# copy binary from builder
COPY --from=builder /build/target/release/mena-rust /usr/sbin/mena-rust

RUN apk add chromium-chromedriver

COPY /docker/pterodactyl/start.sh /start.sh
COPY /docker/pterodactyl/entrypoint.sh /entrypoint.sh

CMD ["/bin/ash", "/entrypoint.sh"]