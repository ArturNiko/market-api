#!/bin/bash

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No color

if [ -f .env ]; then
  echo -e "${CYAN}.env file found. Loading environment variables...${NC}"
  export $(grep -v '^#' .env | xargs)
else
  echo -e "${RED}.env file not found in the project root!${NC}"
  exit 1
fi

if [[ -z "$POSTGRES_USER" || -z "$POSTGRES_DB" || -z "$POSTGRES_PASSWORD" ]]; then
  echo -e "${RED}Error: One or more environment variables are missing in the .env file.${NC}"
  exit 1
fi


# Export password to avoid prompting
export PGPASSWORD="$POSTGRES_PASSWORD"

MIGRATIONS_FOLDER="./database/migrations"

for migration_file in "$MIGRATIONS_FOLDER"/*.sql
do
  echo -e "${CYAN}Applying migration: $migration_file...${NC}"
  ERROR_MESSAGE=$(psql -h localhost -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f "$migration_file" -v ON_ERROR_STOP=1 2>&1)

  # Check if migration exit an error (non-zero status code)
  if [ $? -ne 0 ]; then
    echo -e "${RED}Error applying migration: $migration_file${NC}"
    echo -e "${YELLOW}Error message: $ERROR_MESSAGE${NC}"

    # Unset the password for security
    unset PGPASSWORD

    exit 1
  else
    echo -e "${GREEN}Successfully applied migration: $migration_file${NC}"
  fi
done

echo -e "${GREEN}All migrations have been successfully applied.${NC}"

# Unset the password for security
unset PGPASSWORD
