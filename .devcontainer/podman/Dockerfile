FROM mcr.microsoft.com/devcontainers/base:debian-13

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.96.0 \
    && echo 'source $HOME/.cargo/env' >> /home/vscode/.bashrc

RUN . /home/vscode/.cargo/env && \
    rustup component add rust-analyzer clippy

WORKDIR /workspace
