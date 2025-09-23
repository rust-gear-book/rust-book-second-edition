FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN apt-get install -y git sudo curl file gcc libssl-dev pkg-config

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN echo "[http]\nproxy =" > $HOME/.gitconfig
RUN . $HOME/.cargo/env \
    && export USER=root \
    && cargo install cargo-audit \
    && cargo new hello \
    && cd hello \
    && cargo audit
