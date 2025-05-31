#!/bin/bash

# Default all variables to empty
NODE_NAME=""
NODE_DESCRIPTION=""
NODE_LAT=""
NODE_LNG=""
NODE_ID=""
POSTGRES_PASSWORD=""
ROCKET_SECRET_KEY=""
DATABASE_URL=""
DOCKER_DATABASE_URL=""
ROCKET_ADDRESS=""
ROCKET_PORT=""

ENV_FILE=".env"
OVERWRITE_ENV="n"

if [ -f "$ENV_FILE" ]; then
    echo ".env file already exists."
    read -r -p "Do you want to overwrite it? (y/n) [default: n]: " OVERWRITE_ENV
    OVERWRITE_ENV=${OVERWRITE_ENV:-n}
else
    OVERWRITE_ENV="y"
fi

if [[ "$OVERWRITE_ENV" == "n" ]]; then
    echo "Reusing values from existing .env file..."
    while IFS='=' read -r key value; do
        case "$key" in
            POSTGRES_PASSWORD) POSTGRES_PASSWORD="$value" ;;
            ROCKET_SECRET_KEY) ROCKET_SECRET_KEY="$value" ;;
            DATABASE_URL) DATABASE_URL="$value" ;;
            DOCKER_DATABASE_URL) DOCKER_DATABASE_URL="$value" ;;
            ROCKET_ADDRESS) ROCKET_ADDRESS="$value" ;;
            ROCKET_PORT) ROCKET_PORT="$value" ;;
            NODE_ID) NODE_ID="$value" ;;
            NODE_NAME) NODE_NAME="$value" ;;
            NODE_DESCRIPTION) NODE_DESCRIPTION="$value" ;;
            NODE_LAT) NODE_LAT="$value" ;;
            NODE_LNG) NODE_LNG="$value" ;;
        esac
    done < "$ENV_FILE"

    [[ -z "$POSTGRES_PASSWORD" ]] && echo "WARNING: POSTGRES_PASSWORD is not set. This will be generated and added to .env"
    [[ -z "$ROCKET_SECRET_KEY" ]] && echo "WARNING: ROCKET_SECRET_KEY is not set. This will be generated and added to .env"
    [[ -z "$NODE_ID" ]] && echo "WARNING: NODE_ID is not set. This will be generated and added to .env"
    [[ -z "$DATABASE_URL" ]] && echo "WARNING: DATABASE_URL is not set. This will be generated and added to .env"
    [[ -z "$DOCKER_DATABASE_URL" ]] && echo "WARNING: DOCKER_DATABASE_URL is not set. This will be generated and added to .env"

    if [[ -z "$ROCKET_ADDRESS" ]]; then
        echo "WARNING: ROCKET_ADDRESS is not set. Adding default 0.0.0.0 to .env."
        ROCKET_ADDRESS="0.0.0.0"
        echo >> "$ENV_FILE"
        echo "ROCKET_ADDRESS=$ROCKET_ADDRESS" >> "$ENV_FILE"
    fi

    if [[ -z "$ROCKET_PORT" ]]; then
        echo "WARNING: ROCKET_PORT is not set. Adding default 8000 to .env."
        ROCKET_PORT="8000"
        echo >> "$ENV_FILE"
        echo "ROCKET_PORT=$ROCKET_PORT" >> "$ENV_FILE"
    fi

    [[ -z "$NODE_LAT" ]] && echo "OPTIONAL: NODE_LAT is not set."
    [[ -z "$NODE_LNG" ]] && echo "OPTIONAL: NODE_LNG is not set."
fi

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --node-name)
            NODE_NAME="$2"
            shift 2
            ;;
        --node-description)
            NODE_DESCRIPTION="$2"
            shift 2
            ;;
        --node-id)
            NODE_ID="$2"
            shift 2
            ;;
        --node-lat)
            NODE_LAT="$2"
            shift 2
            ;;
        --node-lng)
            NODE_LNG="$2"
            shift 2
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

# Prompt for required values
[[ -z "$NODE_NAME" ]] && read -r -p "Enter NODE_NAME: " NODE_NAME
[[ -z "$NODE_DESCRIPTION" ]] && read -r -p "Enter NODE_DESCRIPTION: " NODE_DESCRIPTION

# Generate secrets if not defined
if [[ -z "$POSTGRES_PASSWORD" ]]; then
    echo "Generating Postgres password..."
    POSTGRES_PASSWORD=$(LC_ALL=C tr -dc A-Za-z0-9 </dev/urandom | head -c 32)
    [[ "$OVERWRITE_ENV" == "n" ]] && echo -e "\nPOSTGRES_PASSWORD=$POSTGRES_PASSWORD" >> "$ENV_FILE"
    echo "Done!"
fi

if [[ -z "$ROCKET_SECRET_KEY" ]]; then
    echo "Generating Rocket secret key..."
    ROCKET_SECRET_KEY=$(openssl rand -base64 32)
    [[ "$OVERWRITE_ENV" == "n" ]] && echo -e "\nROCKET_SECRET_KEY=$ROCKET_SECRET_KEY" >> "$ENV_FILE"
    echo "Done!"
fi

if [[ -z "$NODE_ID" ]]; then
    echo "Generating node ID..."
    NODE_ID=$(LC_ALL=C tr -dc A-Za-z0-9 </dev/urandom | head -c 32)
    [[ "$OVERWRITE_ENV" == "n" ]] && echo -e "\nNODE_ID=$NODE_ID" >> "$ENV_FILE"
    echo "Done!"
fi

if [[ -z "$DATABASE_URL" ]]; then
    echo "Defining database URL..."
    DATABASE_URL="postgres://postgres:$POSTGRES_PASSWORD@localhost:5444/neighborgoods"
    [[ "$OVERWRITE_ENV" == "n" ]] && echo -e "\nDATABASE_URL=$DATABASE_URL" >> "$ENV_FILE"
    echo "Done!"
fi

if [[ -z "$DOCKER_DATABASE_URL" ]]; then
    echo "Defining docker database URL..."
    DOCKER_DATABASE_URL="postgres://postgres:$POSTGRES_PASSWORD@db:5432/neighborgoods"
    [[ "$OVERWRITE_ENV" == "n" ]] && echo -e "\nDOCKER_DATABASE_URL=$DOCKER_DATABASE_URL" >> "$ENV_FILE"
    echo "Done!"
fi

# Write .env file
if [[ "$OVERWRITE_ENV" == "y" ]]; then
    echo "Writing new values to .env file..."
    cat <<EOF > "$ENV_FILE"
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
fi

# Launch Docker
# docker compose build --no-cache
docker compose up --build -d