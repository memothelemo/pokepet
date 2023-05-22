# Loads Gitpod's postgres workspace image
FROM gitpod/workspace-postgres

# Install neccessary packages that we will use later
RUN sudo apt update \
  && sudo apt install -y \
    redis-server \
    curl \
  && sudo rm -rf /var/lib/apt/lists/*

# Install sqlx CLI
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
