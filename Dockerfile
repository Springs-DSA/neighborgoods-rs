FROM rust:1.86.0

# Install dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev postgresql-client && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install cargo-watch

# Create a new empty project
WORKDIR /app

# Create uploads directory
RUN mkdir -p uploads

# We'll mount the actual code as a volume, but create structure first
COPY Cargo.toml Cargo.lock ./
COPY . .
# COPY Rocket.toml.docker ./Rocket.toml
# COPY cargo_cache /usr/local/cargo/registry
# COPY target_cache /app/target
# COPY ./uploads /app/uploads

# To avoid running as root from here on
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set ownership of working dir and necessary files
RUN chown -R appuser:appuser /app /usr/local/cargo /usr/local/lib /usr/local/bin

USER appuser

# For development hot-reloading, we'll use cargo-watch
# --why shows why restarts happen, --poll ensures changes are detected across volumes
CMD ["cargo", "watch", "--poll", "--why", "-x", "run"]
