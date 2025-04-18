#!/bin/bash

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No color

# get postgres id
POSTGRES_ID=$(docker ps | grep postgres | awk '{print $1}')


if [ -z "$POSTGRES_ID" ]; then
  echo -e "${RED}Error: Postgres container not found.${NC}"
  eixt 1
fi

if [ -f .env ]; then
  echo -e "${CYAN}.env file found. Loading environment variables...${NC}"
  export $(grep -v '^#' .env | xargs)
else
  echo -e "${RED}.env file not found in the project root!${NC}"
  exit 1
fi

if [[ -z "$POSTGRES_USER" || -z "$POSTGRES_DB" ]]; then
  echo -e "${RED}Error: One or more environment variables are missing in the .env file.${NC}"
  exit 1
fi


echo -e "${CYAN}Clearing database...${NC}"
ERROR_MESSAGE=$(docker exec -it "$POSTGRES_ID" psql -U "$POSTGRES_USER" -d postgres -c "DROP DATABASE $POSTGRES_DB;" 2>&1)

if [ $? -ne 0 ]; then
  echo -e "${RED}Error dropping database: $POSTGRES_DB${NC}"
  echo -e "${YELLOW}Error message: $ERROR_MESSAGE${NC}"
  exit 1
fi

ERROR_MESSAGE=$(docker exec -it "$POSTGRES_ID" psql -U "$POSTGRES_USER" -d postgres -c "CREATE DATABASE $POSTGRES_DB;" 2>&1)

if [ $? -ne 0 ]; then
  echo -e "${RED}Error recreating database: $POSTGRES_DB${NC}"
  echo -e "${YELLOW}Error message: $ERROR_MESSAGE${NC}"
  exit 1
fi


echo -e "${GREEN}Database cleared successfully.${NC}"

printf "\n\n"

#execute migrations
./scripts/database/migrate.sh

printf "\n\n"

if [ $? -ne 0 ]; then
  exit 1
fi

#execute seeds
./scripts/database/seed.sh

printf "\n\n"

echo -e "${GREEN}Database reset successfully.${NC}"
