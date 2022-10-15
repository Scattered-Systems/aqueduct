FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y \
    libssl-dev

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM photon as app-base

RUN yum update -y && yum upgrade -y

FROM app-base as latest

COPY --from=builder /app/target/release/conduit /bin/conduit

CMD [ "conduit" ]