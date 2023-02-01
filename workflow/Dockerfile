
FROM golang:1.19

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add wasm32-wasi

RUN apt-get clean && \
rm -rf /var/lib/apt/lists/*whic
RUN apt-get install python3.9 -y
RUN apt-get update
RUN apt-get install -y python3-pip
RUN python3 -m pip install psutil pydantic 
RUN python3 -m pip install tackle-box==0.3.0-beta.5
RUN apt-get install -y clang lldb lld libssl-dev
RUN apt-get clean
RUN apt update
RUN export CC=/usr/bin/clang AR=/usr/bin/llvm-ar-11

WORKDIR /usr/src/
RUN mkdir -p workflow

ADD composer composer/
ADD providers providers/
ADD build/workflow.sh workflow.sh
ADD build/build-test build-test/
RUN cd build-test && cargo build


RUN chmod 777 workflow.sh

RUN apt-get clean && \
rm -rf /var/lib/apt/lists/*whic

ENTRYPOINT ["/usr/src/workflow.sh"]
CMD ["sh", "-c", "cp /usr/src/workflow/workflow.wasm /host/workflow.wasm"]