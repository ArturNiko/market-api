#!/bin/bash

# Color codes
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export CYAN='\033[0;36m'
export NC='\033[0m' # No color

function fetch() {
    if [ -f .env ]; then
      echo -e "\n${CYAN}.env file found. Loading environment variables...${NC}"
      export $(grep -v '^#' .env | xargs)
    else
      echo -e "${RED}.env file not found in the project root!${NC}"
      return 1
    fi

    if [[ -z "$POSTGRES_USER" || -z "$POSTGRES_DB" || -z "$POSTGRES_PASSWORD" ]]; then
      echo -e "${RED}Error: One or more environment variables are missing in the .env file.${NC}"
      return 1
    fi
}

fetch || exit 1

echo "POSTGRES_USER: $POSTGRES_USER POSTGRES_DB: $POSTGRES_DB POSTGRES_PASSWORD: $POSTGRES_PASSWORD"