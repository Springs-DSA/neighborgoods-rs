#!/bin/bash

# Function to prompt for a variable if not provided
prompt_if_empty() {
  local var_name=$1
  local prompt_text=$2
  local var_value=${!var_name}

  if [ -z "$var_value" ]; then
    read -p "$prompt_text: " var_value
    export $var_name="$var_value"
  fi
}

# Read arguments
while [[ "$#" -gt 0 ]]; do
  case $1 in
    --node-name) NODE_NAME="$2"; shift ;;
    --node-description) NODE_DESCRIPTION="$2"; shift ;;
    --node-lat) NODE_LAT="$2"; shift ;;
    --node-lng) NODE_LNG="$2"; shift ;;
    *) echo "Unknown parameter passed: $1"; exit 1 ;;
  esac
  shift
done

# Prompt for required arguments if not passed
prompt_if_empty NODE_NAME "Enter NODE_NAME"
prompt_if_empty NODE_DESCRIPTION "Enter NODE_DESCRIPTION"

# Generate a random password and rocket secret
DB_PASSWORD=$(tr -dc 'A-Za-z0-9_-+=' </dev/urandom | head -c 32)
ROCKET_SECRET_KEY=$(tr -dc 'A-Za-z0-9_-+=' </dev/urandom | head -c 32)
NODE_ID=$(tr -dc 'A-Za-z0-9' </dev/urandom | head -c 32)

# Write all variables to .env
cat > .env <<EOF
POSTGRES_PASSWORD="$DB_PASSWORD"
NODE_ID="$NODE_ID"
NODE_NAME="$NODE_NAME"
NODE_DESCRIPTION="$NODE_DESCRIPTION"
NODE_LAT="$NODE_LAT"
NODE_LNG="$NODE_LNG"
DATABASE_URL="postgres://postgres:password@localhost:5432/neighborgoods"
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=8000
ROCKET_SECRET_KEY="$ROCKET_SECRET_KEY"
EOF

# Start docker compose
docker-compose build --no-cache
docker-compose up -d



