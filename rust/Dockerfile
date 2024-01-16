FROM rust:slim-bookworm
ENV CARGO_TARGET_DIR=/app/target
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch
WORKDIR /app