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

OVERWRITE_ENV="n"
if [[ -f .env ]]; then
    echo "A .env file already exists."
    read -p "Do you want to overwrite it? (y/n) [default: n]: " OVERWRITE_ENV
    OVERWRITE_ENV="${OVERWRITE_ENV:-n}"
else
    OVERWRITE_ENV="y"
fi

# If not overwriting, load existing values
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
    done < .env

    if [[ -z "$POSTGRES_PASSWORD" && -n "$DATABASE_URL" ]]; then
        POSTGRES_PASSWORD=$(echo "$DATABASE_URL" | sed -E 's|.*postgres://[^:]+:([^@]+)@.*|\1|')
        echo "WARNING: POSTGRES_PASSWORD not set, derived from DATABASE_URL"
        echo -e "\nPOSTGRES_PASSWORD=$POSTGRES_PASSWORD" >> .env
    fi

    [[ -z "$ROCKET_SECRET_KEY" ]] && echo "WARNING: ROCKET_SECRET_KEY is not set. This will be generated and added to .env"
    [[ -z "$NODE_ID" ]] && echo "WARNING: NODE_ID is not set. This will be generated and added to .env"
    [[ -z "$DATABASE_URL" ]] && echo "WARNING: DATABASE_URL is not set. This will be generated and added to .env"
    [[ -z "$DOCKER_DATABASE_URL" ]] && echo "WARNING: DOCKER_DATABASE_URL is not set. This will be generated and added to .env"

    if [[ -z "$ROCKET_ADDRESS" ]]; then
        ROCKET_ADDRESS="0.0.0.0"
        echo "WARNING: ROCKET_ADDRESS is not set. Adding default 0.0.0.0 to .env"
        echo -e "\nROCKET_ADDRESS=$ROCKET_ADDRESS" >> .env
    fi

    if [[ -z "$ROCKET_PORT" ]]; then
        ROCKET_PORT="8000"
        echo "WARNING: ROCKET_PORT is not set. Adding default 8000 to .env"
        echo -e "\nROCKET_PORT=$ROCKET_PORT" >> .env
    fi

    [[ -z "$NODE_LAT" ]] && echo "OPTIONAL: NODE_LAT is not set."
    [[ -z "$NODE_LNG" ]] && echo "OPTIONAL: NODE_LNG is not set."
fi

# Parse CLI arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --node-name) NODE_NAME="$2"; shift ;;
        --node-description) NODE_DESCRIPTION="$2"; shift ;;
        --node-id) NODE_ID="$2"; shift ;;
        --node-lat) NODE_LAT="$2"; shift ;;
        --node-lng) NODE_LNG="$2"; shift ;;
        *) echo "Unknown argument: $1"; exit 1 ;;
    esac
    shift
done

# Prompt for required values
[[ -z "$NODE_NAME" ]] && read -p "Enter NODE_NAME: " NODE_NAME
[[ -z "$NODE_DESCRIPTION" ]] && read -p "Enter NODE_DESCRIPTION: " NODE_DESCRIPTION

# Generate POSTGRES_PASSWORD if needed
if [[ -z "$POSTGRES_PASSWORD" ]]; then
    echo "Generating Postgres password..."
    POSTGRES_PASSWORD=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 32)
    if [[ "$OVERWRITE_ENV" == "n" ]]; then
        echo -e "\nPOSTGRES_PASSWORD=$POSTGRES_PASSWORD" >> .env
    fi
    echo "Done!"
fi

# Generate ROCKET_SECRET_KEY if needed
if [[ -z "$ROCKET_SECRET_KEY" ]]; then
    echo "Generating Rocket secret key..."
    ROCKET_SECRET_KEY=$(openssl rand -base64 32)
    if [[ "$OVERWRITE_ENV" == "n" ]]; then
        echo -e "\nROCKET_SECRET_KEY=$ROCKET_SECRET_KEY" >> .env
    fi
    echo "Done!"
fi

# Generate NODE_ID if needed
if [[ -z "$NODE_ID" ]]; then
    echo "Generating NODE_ID..."
    NODE_ID=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 32)
    if [[ "$OVERWRITE_ENV" == "n" ]]; then
        echo -e "\nNODE_ID=$NODE_ID" >> .env
    fi
    echo "Done!"
fi

# Generate DATABASE_URL if not set
if [[ -z "$DATABASE_URL" ]]; then
    DATABASE_URL="postgres://postgres:$POSTGRES_PASSWORD@localhost:5444/neighborgoods"
    if [[ "$OVERWRITE_ENV" == "n" ]]; then
        echo -e "\nDATABASE_URL=$DATABASE_URL" >> .env
    fi
fi

# Generate DOCKER_DATABASE_URL if not set
if [[ -z "$DOCKER_DATABASE_URL" ]]; then
    DOCKER_DATABASE_URL="postgres://postgres:$POSTGRES_PASSWORD@db:5432/neighborgoods"
    if [[ "$OVERWRITE_ENV" == "n" ]]; then
        echo -e "\nDOCKER_DATABASE_URL=$DOCKER_DATABASE_URL" >> .env
    fi
fi

# Overwrite .env if opted in
if [[ "$OVERWRITE_ENV" == "y" ]]; then
    echo "Writing new values to .env file..."
    cat > .env <<EOF
POSTGRES_PASSWORD=$POSTGRES_PASSWORD
NODE_ID=$NODE_ID
NODE_NAME="$NODE_NAME"
NODE_DESCRIPTION="$NODE_DESCRIPTION"
NODE_LAT=$NODE_LAT
NODE_LNG=$NODE_LNG
DATABASE_URL=$DATABASE_URL
DOCKER_DATABASE_URL=$DOCKER_DATABASE_URL
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
ROCKET_SECRET_KEY=$ROCKET_SECRET_KEY
EOF
fi

# Launch docker
docker compose up --build -d