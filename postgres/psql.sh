#!/bin/bash
# Note: The postgres container needs to running for this
docker-compose exec lildbtool-postgres psql -U postgres_user example_database
