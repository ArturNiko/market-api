#!/bin/bash

MIGRATIONS_FOLDER="./database/migrations"


TIMESTAMP=$(date +%Y%m%d%H%M%S)

echo "Enter migration name (e.g., 'create_users_table'): "
read MIGRATION_NAME

MIGRATION_FILE="$MIGRATIONS_FOLDER/$TIMESTAMP"_"$MIGRATION_NAME.sql"

touch "$MIGRATION_FILE"

echo "Migration file created: $MIGRATION_FILE"
echo "You can now add your SQL commands in the migration file."