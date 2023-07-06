FROM clux/muslrust:1.70.0 as builder

WORKDIR /workspace
COPY . .

# clux/muslrust is not perfect, we need to install missing dependencies
# that rustc needs in order to compile with musl successfully (sort of)
RUN rustup target add x86_64-unknown-linux-musl

# rustfmt
RUN rustup component add 1.70.0-x86_64-unknown-linux-gnu

# Install sqlx-cli
RUN cargo install sqlx-cli --version "0.7.0" --no-default-features --features rustls,postgres

# Compile the project first or ignore for now
RUN cargo build || :
