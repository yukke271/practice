FROM rust:slim-bookworm
ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rls rust-analysis rust-src rustfmt clippy && \
    cargo install cargo-edit cargo-watch
WORKDIR /app