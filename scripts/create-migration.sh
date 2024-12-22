#!/bin/bash

# Define the migrations folder
MIGRATIONS_FOLDER="./database/migrations"

# Create a timestamp for the migration filename (YYYYMMDDHHMMSS)
TIMESTAMP=$(date +%Y%m%d%H%M%S)

# Ask for the migration name (e.g., 'create_users_table')
echo "Enter migration name (e.g., 'create_users_table'): "
read MIGRATION_NAME

# Create the migration file with the timestamp and migration name
MIGRATION_FILE="$MIGRATIONS_FOLDER/$TIMESTAMP"_"$MIGRATION_NAME.sql"

# Create an empty migration file
touch "$MIGRATION_FILE"

# Provide instructions
echo "Migration file created: $MIGRATION_FILE"
echo "You can now add your SQL commands in the migration file."

# Open the migration file with your default editor (optional)
# Uncomment the next line if you'd like the editor to open automatically
# nano "$MIGRATION_FILE"
