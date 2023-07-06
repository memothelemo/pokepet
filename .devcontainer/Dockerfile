FROM clux/muslrust:1.70.0 as builder

WORKDIR /workspace
COPY . .

# Update and install required dependencies, not sure which
# one it is redundant
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

# Compile the project first or ignore for now
RUN cargo build || :
