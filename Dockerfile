FROM ghcr.io/rust-lang/crates-build-env/linux:latest
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
WORKDIR /app
COPY . .
RUN . $HOME/.cargo/env && cargo doc
