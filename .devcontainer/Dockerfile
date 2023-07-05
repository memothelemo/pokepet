FROM rust:1.70.0-slim-bullseye as builder

WORKDIR /workspace

# Update and install required dependencies
RUN apt-get update && \
  apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  build-essential \
  openssl \
  libpq-dev \
  && sudo rm -rf /var/lib/apt/lists/*

# Install diesel-cli
RUN cargo install diesel_cli --no-default-features --features postgres
