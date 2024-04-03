FROM rust:latest as build
 
RUN mkdir consumer
WORKDIR consumer
 
RUN rustup default 1.74.1
RUN apt-get update
 
RUN apt install make clang pkg-config libssl-dev -y
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-wasi
 
ADD . .
RUN cargo update
RUN cargo build --release
 
## node
RUN printf 'y\n1\n\1n' | apt install nodejs
RUN apt install -y npm
 
RUN npm install -g ssb-server@15.3.0
 
HEALTHCHECK --interval=30s --timeout=30s --start-period=10s --retries=10 \
  CMD ssb-server whoami || exit 1
ENV HEALING_ACTION RESTART
 
EXPOSE 8008
EXPOSE 8080
 
# Start the ssb-server in the background  
ENTRYPOINT ["/bin/sh", "-c", "./entrypoint.sh" ]