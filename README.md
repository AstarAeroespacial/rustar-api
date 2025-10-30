# Rust API Server

A Rust API server built with Actix-web that includes configuration management, database integration, MQTT messaging, and OpenAPI documentation. The application uses PostgreSQL (Supabase or local) for data persistence and Mosquitto for MQTT communication.

## Architecture

The application follows a clean architecture pattern with the following layers:

-   **Routes**: Handle HTTP requests and responses
-   **Services**: Business logic layer for exchanging messages and accessing telemetry
-   **Repository**: Data access layer with database abstraction
-   **Messaging**: Communication layer for interacting with ground stations
-   **Models**: Data structures and DTOs

## Technology Stack

-   `actix_web` and `utoipa` for the HTTP server and API documentation respectively
-   `rumqttc` for MQTT integration
-   `sqlx` for postgres integration
-   The app was developed with `psql` for the database and `mosquitto` for the MQTT broker

## Configuration

The server uses a `config.toml` file for configuration. The following sections are available:

### Server Configuration

-   `host`: Server host address (default: 127.0.0.1)
-   `port`: Server port (default: 8080)

### Database Configuration

-   `url`: Database connection string
-   `pool_size`: Connection pool size

### Message Broker Configuration

-   `host`: Message broker address
-   `port`: Port for the connection
-   `keep_alive`: keepalive message interval

## Environment Variables

The server uses environment variables with the `API_` prefix for configuration. You can set these in a `.env` file (recommended) or export them directly.

**Important:** Copy `.env.example` to `.env` and configure for your environment:

```bash
cp .env.example .env
# Edit .env with your settings
```

### Configuration Options

`.env.example` provides two configurations:

1. **Local Development (Docker)** - Uses containerized PostgreSQL
2. **Supabase (Remote)** - Uses hosted Supabase database

Key variables:

-   `API_SERVER_HOST` / `API_SERVER_PORT` - Server binding
-   `API_DATABASE_URL` / `DATABASE_URL` - Database connection (both should point to same DB)
-   `API_SKIP_MIGRATIONS` - Set to `true` when using Supabase (migrations already applied)
-   `API_MESSAGE_BROKER_HOST` / `API_MESSAGE_BROKER_PORT` - MQTT broker connection

## Quick Start with Docker Compose

The easiest way to run the application locally with all dependencies:

```bash
# 1. Clone and enter the repository
git clone <repository-url>
cd rustar-api

# 2. Copy and configure environment
cp .env.example .env
# Uncomment "Option 1: Local PostgreSQL" in .env for local development

# 3. Start all services (PostgreSQL, Mosquitto, API)
docker compose up -d

# 4. Check logs
docker compose logs -f api

# 5. Verify database tables were created
docker compose exec postgres psql -U postgres -d rustar-api -c "\dt"
```

The API will be available at `http://localhost:9090`

### Stopping Services

```bash
# Stop services but keep data
docker compose down

# Stop and remove all data (fresh start)
docker compose down -v
```

## Setup for Local Development (without Docker)

If you prefer to run the API directly without Docker:

### MQTT Broker Setup

**Option A: Use Docker Compose** (Recommended)

-   Mosquitto is included in `docker-compose.yaml` and starts automatically

**Option B: Manual Installation**

1. Install [mosquitto](https://www.mosquitto.org/download/)
2. Run the broker: `mosquitto -p 1883`
3. Update `.env` with the correct host and port

### Database Setup

**Option A: Use Supabase (Production/Remote)**

1. Set your Supabase connection string in `.env`:

    ```bash
    API_DATABASE_URL=postgresql://postgres:YOUR_PASSWORD@db.gxrcklaazsihvgbxxddy.supabase.co:5432/postgres?sslmode=require
    DATABASE_URL=postgresql://postgres:YOUR_PASSWORD@db.gxrcklaazsihvgbxxddy.supabase.co:5432/postgres?sslmode=require
    API_SKIP_MIGRATIONS=true  # Migrations already applied on Supabase
    ```

2. The schema is already deployed on Supabase. No additional setup needed.

**Option B: Use Local PostgreSQL**

1. Install [PostgreSQL](https://www.postgresql.org/download/) or use Docker:

    ```bash
    docker compose up -d postgres
    ```

2. Install [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md):

    ```bash
    cargo install sqlx-cli --no-default-features --features postgres
    ```

3. Set local database URL in `.env`:

    ```bash
    API_DATABASE_URL=postgresql://postgres:postgres@localhost:5433/rustar-api
    DATABASE_URL=postgresql://postgres:postgres@localhost:5433/rustar-api
    API_SKIP_MIGRATIONS=false  # Run migrations on startup
    ```

4. Migrations run automatically when the API starts. To run manually:

    ```bash
    sqlx migrate run
    ```

5. (Optional) Generate test data:
    ```bash
    cargo run --bin seed_data
    ```

### Running the Server

1. Ensure `.env` is configured (see Environment Variables section above)

2. Run the API:

    ```bash
    cargo run --bin api
    ```

3. The server will:
    - Connect to the database
    - Run migrations (unless `API_SKIP_MIGRATIONS=true`)
    - Connect to MQTT broker
    - Start listening on the configured host:port

## API Endpoints

### Telemetry

-   `GET /api/telemetry/{satellite}/latest?amount=10` - Get latest telemetry data for a satellite
-   `GET /api/telemetry/{satellite}/history?startTime=<unix>&endTime=<unix>` - Get historic telemetry data

### Tracking

-   `GET /api/tracking/position?sat_id=<id>&gs_id=<id>&epoch=<unix>` - Calculate satellite position using TLE

### Control

-   `POST /api/control/command` - Send commands to satellite via MQTT

### Configuration & Documentation

-   `GET /config` - View current server configuration
-   `GET /swagger-ui/` - Interactive OpenAPI documentation

## Database Schema

The application uses the following tables (see `migrations/` for details):

-   `satellites` - Satellite information and TLE data
-   `ground_stations` - Ground station locations (latitude, longitude, altitude)
-   `telemetry` - Telemetry data from satellites
-   `jobs` - Scheduled communication jobs between satellites and ground stations
-   `jobs_status_updates` - Job execution status tracking

## Database Migrations

### Creating a New Migration

When you need to modify the database schema:

```bash
# 1. Create a new migration file
sqlx migrate add your_migration_name

# 2. Edit the generated file in migrations/ with your SQL
# Example: migrations/20251026184727_your_migration_name.sql

# 3. Apply locally (automatic on Docker restart or manually)
sqlx migrate run

# For Docker:
docker compose build api --no-cache  # Rebuild with new migration
docker compose up -d                  # Restart services
```

### Applying Migrations to Supabase

After creating and testing a migration locally, apply it to the production Supabase database:

```bash
# 1. Export environment variables from .env
export $(cat .env | grep -v '^#' | xargs)

# 2. Run the migration script
./scripts/apply_migrations_to_supabase.sh

# Or with automatic confirmation:
AUTO_CONFIRM=1 ./scripts/apply_migrations_to_supabase.sh
```

The script will:

-   ✅ Detect which migrations are already applied in Supabase
-   ✅ Show you the pending migrations
-   ✅ Apply only the new migrations
-   ✅ Register them in `_sqlx_migrations` table
-   ✅ Skip migrations that would cause conflicts (tables already exist)

### Migration Workflow Example

Complete workflow for a schema change:

```bash
# 1. Create migration
sqlx migrate add add_new_column

# 2. Edit migrations/TIMESTAMP_add_new_column.sql
# Add your SQL: ALTER TABLE satellites ADD COLUMN status TEXT;

# 3. Test locally
docker compose build api --no-cache
docker compose up -d
docker compose logs -f api  # Verify migration ran

# 4. Apply to Supabase
export $(cat .env | grep -v '^#' | xargs)
./scripts/apply_migrations_to_supabase.sh

# 5. Commit changes
git add migrations/
git commit -m "Add new column to satellites table"
```

### Exporting Schema from Supabase (Advanced)

If you need to export the current Supabase schema (e.g., to create a fresh local copy):

```bash
# 1. Export full schema from Supabase
./scripts/export_supabase_schema.sh

# 2. Extract only public schema (removes auth, storage, etc.)
./scripts/extract_public_schema.sh

# Result: public_schema_only.sql
```

## Development

The server is structured with:

-   `src/main.rs` - Main application entry point
-   `src/config.rs` - Configuration management
-   `src/models/` - Data models and DTOs
-   `src/repository/` - Database access layer
-   `src/services/` - Business logic layer
-   `src/routes/` - HTTP route handlers
-   `src/database/` - Database connection management
-   `src/messaging/` - MQTT broker integration
-   `migrations/` - Database schema migrations
-   `scripts/` - Utility scripts for schema export and management

### Building for Production

```bash
# Build release binary
cargo build --release

# Or build Docker image
docker build -t rustar-api .
```

### Running Tests

```bash
cargo test
```

## Troubleshooting

### Migration Errors

If you see migration conflicts:

```bash
# Clean and restart
docker compose down -v
docker compose up -d
```

### Connection Refused to Database

-   Verify PostgreSQL is running: `docker compose ps`
-   Check `.env` has correct `API_DATABASE_URL`
-   For Supabase, ensure password is correct and `?sslmode=require` is included

### Removing sqlx warnings (invalid port number / macro errors)

If you see sqlx macro errors in the editor (for example `invalid port number` or macro expansion failures from `query!`/`query_as!`), it's usually because the language server or `cargo check` can't connect to the configured `DATABASE_URL`. Here are three safe options to remove those warnings:

Use a local PostgreSQL for editor checks (recommended for development)

1. Ensure your local DB is running (Docker Compose is easiest):

```bash
docker compose up -d postgres
```

2. Set the local DB URL in your `.env` (example already configured for local dev):

```bash
# in .env
DATABASE_URL=postgresql://postgres:postgres@localhost:5433/postgres
API_DATABASE_URL=postgresql://postgres:postgres@localhost:5433/postgres
```

3. Prepare the sqlx offline cache (this writes `.sqlx/`):

```bash
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/postgres" cargo sqlx prepare
```

4. Run a local check to validate macros:

```bash
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/postgres" cargo check
```

### MQTT Connection Issues

-   Verify Mosquitto is running: `docker compose logs mosquitto`
-   Check `API_MESSAGE_BROKER_HOST` and `API_MESSAGE_BROKER_PORT` in `.env`
