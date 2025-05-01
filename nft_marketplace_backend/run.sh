#!/bin/bash

# Exit immediately on error
set -e

# Load environment variables
if [ -f .env ]; then
  export $(cat .env | xargs)
fi

# Run SQL migrations
echo "Running database migrations..."
sqlx migrate run

# Start the FastAPI server
echo "Starting FastAPI server..."
uvicorn main:app --reload
