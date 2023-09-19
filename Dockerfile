FROM rust:1.71-slim-buster
RUN apt-get update -y && apt-get install git -y
RUN git clone -b testnet3 \
    https://github.com/puzzlehq/aleo-rust.git \
    --depth 1
WORKDIR aleo-rust
RUN ["chmod", "+x", "./build_ubuntu.sh"]
RUN ./build_ubuntu.sh
EXPOSE 3033/tcp
EXPOSE 4133/tcp
RUN cargo install --path . --locked
WORKDIR rust/develop
CMD cargo run --bin aleo-develop start -d
