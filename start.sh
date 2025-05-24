#!/bin/bash

# Default all variables to empty
NODE_NAME=""
NODE_DESCRIPTION=""
NODE_LAT=""
NODE_LNG=""
NODE_ID=""
POSTGRES_PASSWORD=""
ROCKET_SECRET_KEY=""

ENV_FILE=".env"

# Load existing values if .env exists and user doesn't want to overwrite
OVERWRITE_ENV="n"
if [[ -f "$ENV_FILE" ]]; then
    read -p "A .env file already exists. Do you want to overwrite it? (y/n) [default: n]: " OVERWRITE_ENV
    OVERWRITE_ENV=${OVERWRITE_ENV,,}  # to lowercase
    [[ -z "$OVERWRITE_ENV" ]] && OVERWRITE_ENV="n"

    if [[ "$OVERWRITE_ENV" == "n" ]]; then
        echo "Reusing values from existing .env file..."
        while IFS='=' read -r key value; do
            key=$(echo "$key" | tr -d '[:space:]')
            value=$(echo "$value" | sed -E 's/^"(.*)"$/\1/') # remove surrounding quotes
            case "$key" in
                POSTGRES_PASSWORD) POSTGRES_PASSWORD="$value" ;;
                ROCKET_SECRET_KEY) ROCKET_SECRET_KEY="$value" ;;
                NODE_ID) NODE_ID="$value" ;;
                NODE_NAME) NODE_NAME="$value" ;;
                NODE_DESCRIPTION) NODE_DESCRIPTION="$value" ;;
                NODE_LAT) NODE_LAT="$value" ;;
                NODE_LNG) NODE_LNG="$value" ;;
            esac
        done < "$ENV_FILE"
    fi
fi

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --node-name) NODE_NAME="$2"; shift 2 ;;
        --node-description) NODE_DESCRIPTION="$2"; shift 2 ;;
        --node-id) NODE_ID="$2"; shift 2 ;;
        --node-lat) NODE_LAT="$2"; shift 2 ;;
        --node-lng) NODE_LNG="$2"; shift 2 ;;
        *) echo "Unknown argument: $1"; exit 1 ;;
    esac
done

# Prompt for required values if not set
[[ -z "$NODE_NAME" ]] && read -p "Enter NODE_NAME: " NODE_NAME
[[ -z "$NODE_DESCRIPTION" ]] && read -p "Enter NODE_DESCRIPTION: " NODE_DESCRIPTION

# Setup random ID generator
generate_random_string() {
    local length=$1
    tr -dc 'A-Za-z0-9' </dev/urandom | head -c "$length"
}

# Generate secrets if not already loaded
[[ -z "$POSTGRES_PASSWORD" ]] && POSTGRES_PASSWORD=$(generate_random_string 32)
[[ -z "$NODE_ID" ]] && NODE_ID=$(generate_random_string 32)

# Generate base64 ROCKET_SECRET_KEY
if [[ -z "$ROCKET_SECRET_KEY" ]]; then
    ROCKET_SECRET_KEY=$(head -c 32 /dev/urandom | base64)
fi

# Write to .env if overwriting
if [[ "$OVERWRITE_ENV" == "y" ]]; then
    echo "Writing new values to $ENV_FILE..."
    cat > "$ENV_FILE" <<EOF
POSTGRES_PASSWORD=$POSTGRES_PASSWORD
NODE_ID=$NODE_ID
NODE_NAME="$NODE_NAME"
NODE_DESCRIPTION="$NODE_DESCRIPTION"
NODE_LAT=$NODE_LAT
NODE_LNG=$NODE_LNG
DATABASE_URL=postgres://postgres:$POSTGRES_PASSWORD@localhost:5444/neighborgoods
DOCKER_DATABASE_URL=postgres://postgres:$POSTGRES_PASSWORD@db:5432/neighborgoods
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
ROCKET_SECRET_KEY=$ROCKET_SECRET_KEY
EOF
else
    echo "Skipping .env overwrite. Using existing values."
fi

# Launch Docker
docker compose up --build -d