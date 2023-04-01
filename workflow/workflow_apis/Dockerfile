FROM kkshanith/ubuntu_with_rust_wasm:0.1

WORKDIR /usr/src/
RUN mkdir -p workflow_api

ADD src src/
ADD migrations migrations/
ADD Cargo.toml Cargo.toml
ADD diesel.toml diesel.toml
ADD .env .env

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y postgresql postgresql-contrib libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
WORKDIR /usr/src/workflow_api
RUN cargo build --release

ENTRYPOINT ["/usr/src/workflow_api/target/release/workflow_apis"]