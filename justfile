# load .env file
set dotenv-load

# Creates the database specified in your DATABASE_URL
db-create:
  sqlx database create --database-url $DATABASE_URL

# Drops the database specified in your DATABASE_URL
db-drop:
  sqlx database drop --database-url $DATABASE_URL

# Drops the database specified in your DATABASE_URL, re-creates it, and runs any pending migrations
db-reset:
  sqlx database reset --source $MIGRATIONS_DIR

# Create a new migration with the given description
db-new-migration MIGRATION_NAME:
  sqlx migrate add -r {{MIGRATION_NAME}} --source $MIGRATIONS_DIR

# Run all pending migrations
db-run-migrations:
  sqlx migrate run --source $MIGRATIONS_DIR

# Revert the latest migration with a down file
db-revert-migration:
  sqlx migrate revert --source $MIGRATIONS_DIR
