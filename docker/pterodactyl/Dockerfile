# get rust image
FROM rust AS builder

# set working directory
WORKDIR /builder

# copy source code
COPY . .

# build binary with release profile
RUN rustup update
RUN cargo build --release --bin mena-rust

# get other rust image for final image
FROM debian:latest AS runtime

# create user
RUN adduser --disabled-password --home /home/container container

# install chromium & chromium-driver
RUN apt-get update
RUN apt-get install chromium chromium-driver ash -y

# set user to container
# will uncomment after release.....
# USER container

# set working directory
WORKDIR /home/container
ENV USER container
ENV HOME /home/container
ENV MENA_CONFIG /home/container/config.toml

# copy binary from builder
COPY --from=builder /builder/target/release/mena-rust /usr/bin/mena-rust

#COPY /docker/pterodactyl/start.sh /start.sh
COPY /docker/pterodactyl/entrypoint.sh /entrypoint.sh

CMD ["/bin/ash", "/entrypoint.sh"]