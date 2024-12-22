#!/bin/bash

if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
else
  echo ".env file not found in the project root!"
  exit 1
fi


if [[ -z "$POSTGRES_USER" || -z "$POSTGRES_DB" || -z "$POSTGRES_PASSWORD" ]]; then
  echo "Error: One or more environment variables are missing in the .env file."
  exit 1
fi


# Export password to avoid prompting
export PGPASSWORD="$POSTGRES_PASSWORD"

MIGRATIONS_FOLDER="./database/migrations"

for migration_file in "$MIGRATIONS_FOLDER"/*.sql
do
  ERROR_MESSAGE=$(psql -h localhost -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f "$migration_file" -v ON_ERROR_STOP=1 2>&1)

  # check if migration returned an error (non-zero status code)
  if [ $? -ne 0 ]; then
    echo "Error applying migration: $migration_file"
    echo "Error message: $ERROR_MESSAGE"
    exit 1
  else
    echo "Successfully applied migration: $migration_file"
  fi
done

echo "All migrations have been successfully applied."

# Unset the password for security
unset PGPASSWORD
