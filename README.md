# Rust REST API Boilerplate

A production-ready starter template for building REST APIs in Rust.
Built with **Axum**, **SeaORM**, and **Supabase Auth** (Google OAuth + email/password).

Use this as a foundation for new projects — clone it, rename it, and start adding your own
entities and routes.

## Tech Stack

| Crate                                        | Role                                                     |
| -------------------------------------------- | -------------------------------------------------------- |
| [Axum](https://docs.rs/axum)                 | HTTP framework (routing, extractors, middleware)         |
| [SeaORM](https://www.sea-ql.org/SeaORM/)     | Async ORM for PostgreSQL (entities, queries, migrations) |
| [Tokio](https://tokio.rs/)                   | Async runtime                                            |
| [jsonwebtoken](https://docs.rs/jsonwebtoken) | Supabase JWT validation (HS256)                          |
| [validator](https://docs.rs/validator)       | Declarative request body validation via derive macros    |
| [tower-http](https://docs.rs/tower-http)     | CORS and HTTP request tracing layers                     |
| [tracing](https://docs.rs/tracing)           | Structured logging                                       |
| [thiserror](https://docs.rs/thiserror)       | Ergonomic error type definitions                         |
| [dotenvy](https://docs.rs/dotenvy)           | `.env` file loading                                      |
| [chrono](https://docs.rs/chrono)             | Date/time types for timestamps                           |
| [uuid](https://docs.rs/uuid)                 | UUID generation and serialization                        |

## Project Structure

```
.
├── Cargo.toml                  # Workspace root — app dependencies
├── Makefile                    # Dev workflow shortcuts (see below)
├── docker-compose.yml          # Local PostgreSQL 16
├── .env.example                # Environment variable template
│
├── migration/                  # SeaORM migration crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Migrator — registers all migrations
│       ├── main.rs             # CLI entrypoint (sea-orm-migration)
│       └── m20250223_000001_create_profiles_table.rs
│
└── src/
    ├── main.rs                 # Bootstrap: config → DB → router → serve
    ├── config.rs               # Typed Config struct loaded from env vars
    ├── db.rs                   # Database connection + auto-migration on startup
    ├── errors.rs               # AppError enum → JSON error responses
    │
    ├── extractors/
    │   ├── auth.rs             # AuthUser — validates Supabase JWT, extracts user identity
    │   └── validated_json.rs   # ValidatedJson<T> — deserialize + validate in one step
    │
    ├── models/
    │   └── profile.rs          # SeaORM entity for the `profiles` table
    │
    ├── routes/
    │   ├── mod.rs              # Health check + shared ProfileResponse DTO
    │   ├── auth.rs             # POST /auth/callback — upsert profile after login
    │   └── user.rs             # CRUD endpoints for user profiles
    │
    └── services/
        └── user.rs             # Profile business logic (find, create, update, delete)
```

## Auth Architecture

Authentication is fully delegated to **Supabase**. This API never handles passwords
or OAuth flows directly — it only validates the JWT that Supabase issues after login.

```
Client → Supabase (Google OAuth / email+password) → receives JWT
Client → This API (Authorization: Bearer <jwt>) → validates token → serves request
```

The `AuthUser` extractor reads the `Authorization` header, verifies the token using
your Supabase project's JWT secret (HS256), and makes the user's identity available
to any handler that includes it as a parameter.

## Database Strategy

- **Development**: local PostgreSQL via Docker (fast, works offline)
- **UAT / Production**: Supabase's hosted PostgreSQL

Both use the same schema and migrations. Switch between them by changing `DATABASE_URL`
in your `.env` file. Migrations run automatically on every application startup.

The boilerplate uses a `profiles` table to store app-specific user data, linked to
Supabase's internal user ID via the `auth_id` column.

## Prerequisites

- **Rust** (stable toolchain) — install via [rustup.rs](https://rustup.rs/)
- **Docker** and **Docker Compose** — for the local PostgreSQL database
- **A Supabase project** — free tier at [supabase.com](https://supabase.com)

## Getting Started

### 1. Clone and configure

```bash
git clone <your-repo-url> my-api
cd my-api
make setup      # copies .env.example → .env
```

Edit `.env` and fill in your Supabase credentials:

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/rest_api
SUPABASE_JWT_SECRET=<your-jwt-secret>
SUPABASE_URL=https://<your-project>.supabase.co
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
```

### 2. Supabase configuration

In your Supabase dashboard:

1. **Get the JWT secret**: Settings → API → JWT Secret → copy into `SUPABASE_JWT_SECRET`
2. **Get the project URL**: Settings → API → Project URL → copy into `SUPABASE_URL`
3. **Enable Google OAuth** (optional): Authentication → Providers → Google → toggle on
   and provide your Google Client ID and Client Secret

### 3. Start the database and run

```bash
make db         # starts PostgreSQL in Docker
make dev        # builds and runs the API with debug logging
```

The server starts on `http://localhost:3000`. Migrations run automatically.

### 4. Verify it works

```bash
curl http://localhost:3000/health
# → {"status":"ok"}
```

## API Endpoints

### Public

| Method | Path      | Description                 |
| ------ | --------- | --------------------------- |
| `GET`  | `/health` | Database connectivity check |

### Protected (require `Authorization: Bearer <supabase-jwt>`)

| Method   | Path             | Description                             |
| -------- | ---------------- | --------------------------------------- |
| `POST`   | `/auth/callback` | Upsert a profile after Supabase login   |
| `GET`    | `/users/me`      | Get the authenticated user's profile    |
| `PUT`    | `/users/me`      | Update the authenticated user's profile |
| `DELETE` | `/users/me`      | Delete the authenticated user's profile |
| `GET`    | `/users/{id}`    | Get any user's profile by UUID          |

### Error format

All errors return a consistent JSON structure:

```json
{
  "error": {
    "status": 401,
    "message": "missing authorization header"
  }
}
```

## Makefile Targets

| Target                         | Description                                             |
| ------------------------------ | ------------------------------------------------------- |
| `make setup`                   | Copy `.env.example` to `.env` (skips if already exists) |
| `make db`                      | Start PostgreSQL via Docker Compose                     |
| `make db-stop`                 | Stop the database container                             |
| `make dev`                     | Run the API with `RUST_LOG=debug`                       |
| `make build`                   | Compile a release binary                                |
| `make check`                   | Run `cargo check` (fast type-checking)                  |
| `make test`                    | Run all tests                                           |
| `make fmt`                     | Format code with `rustfmt`                              |
| `make lint`                    | Run `clippy` lints                                      |
| `make migrate`                 | Run **all** pending migrations                          |
| `make migrate-new NAME=<name>` | Create a new migration file                             |
| `make migrate-one`             | Run only the **next** pending migration                 |
| `make migrate-down`            | Rollback **all** applied migrations                     |
| `make migrate-rollback`        | Rollback only the **last** applied migration            |
| `make clean`                   | Remove build artifacts                                  |

## Migrations

All migration commands use the `migration` crate CLI. You can invoke them via `make` targets or `cargo run -p migration` directly.

### Create a new migration

```bash
make migrate-new NAME=create_users_table
# or directly:
cargo run -p migration -- new create_users_table
```

This generates a file like `migration/src/20260227_000001_create_users_table.rs` with the format
`YYYYMMDD_SERIAL_name.rs` (serial is sequential per date, zero-padded to 6 digits).

The file is pre-filled with `up` and `down` stubs:

```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // TODO: implement the migration
}

async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // TODO: implement the rollback
}
```

After editing, **register the new migration** in `migration/src/lib.rs`:

```rust
mod m20260227_000001_create_users_table;

pub struct Migrator;
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250223_000001_create_profiles_table::Migration),
            Box::new(m20260227_000001_create_users_table::Migration), // ← add here
        ]
    }
}
```

### Run all pending migrations

```bash
make migrate
# or directly:
cargo run -p migration -- up
```

### Run only the next migration

```bash
make migrate-one
# or directly:
cargo run -p migration -- up -n 1
```

### Rollback the last migration

```bash
make migrate-rollback
# or directly:
cargo run -p migration -- down -n 1
```

### Rollback all migrations

```bash
make migrate-down
# or directly:
cargo run -p migration -- down
```

## Extending the Boilerplate

To add a new entity (e.g. `posts`):

1. **Migration** — run `make migrate-new NAME=create_posts_table`, implement `up`/`down`,
   then register the struct in `migration/src/lib.rs`
2. **Model** — add `src/models/post.rs` with the SeaORM entity and re-export it from `src/models/mod.rs`
3. **Service** — add `src/services/post.rs` with your CRUD functions and re-export from `src/services/mod.rs`
4. **Routes** — add `src/routes/post.rs`, create a `router()` function, and nest it in `src/main.rs`
5. **Validation** — define request DTOs with `#[derive(Deserialize, Validate)]` and use
   `ValidatedJson<T>` as the extractor in your handlers

## License

MIT
