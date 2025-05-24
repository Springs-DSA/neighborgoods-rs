FROM rust:1.86.0

# Install dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install cargo-watch

# Create a new empty project
WORKDIR /app

# We'll mount the actual code as a volume, but create structure first
COPY Cargo.toml .
COPY Cargo.lock .

# Copy the Docker-specific Rocket.toml
COPY Rocket.toml.docker ./Rocket.toml

# Create directories
RUN mkdir -p uploads
RUN mkdir -p target/debug

# To avoid running as root from here on
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set ownership of working dir and necessary files
RUN chown -R appuser:appuser /app /usr/local/cargo /usr/local/lib /usr/local/bin

USER appuser

# For development hot-reloading, we'll use cargo-watch
# --why shows why restarts happen, --poll ensures changes are detected across volumes
CMD ["cargo", "watch", "--poll", "--why", "-x", "run"]
