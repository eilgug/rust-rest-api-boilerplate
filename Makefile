.PHONY: setup db db-stop dev build check test fmt lint migrate migrate-new migrate-down migrate-rollback clean

## ── Environment ──────────────────────────────────────────────

setup: ## Copy .env.example to .env (skips if .env already exists)
	@test -f .env && echo ".env already exists, skipping" || (cp .env.example .env && echo "Created .env from .env.example — edit it with your credentials")

## ── Database ─────────────────────────────────────────────────

db: ## Start PostgreSQL via Docker Compose
	docker compose up -d

db-stop: ## Stop the database container
	docker compose down

## ── Development ──────────────────────────────────────────────

dev: ## Run the API with debug logging
	RUST_LOG=debug cargo run

build: ## Compile a release binary
	cargo build --release

check: ## Fast type-check without producing a binary
	cargo check

test: ## Run all tests
	cargo test

## ── Code Quality ─────────────────────────────────────────────

fmt: ## Format code with rustfmt
	cargo fmt

lint: ## Run clippy lints
	cargo clippy -- -D warnings

## ── Migrations ───────────────────────────────────────────────

migrate: ## Run all pending migrations (alias for migrate-up)
	cargo run -p migration -- up

migrate-new: ## Create a new migration file — usage: make migration-new NAME=create_users_table
	@test -n "$(NAME)" || (echo "Usage: make migration-new NAME=<migration_name>"; exit 1)
	cargo run -p migration -- new $(NAME)

migrate-one: ## Run only the next pending migration
	cargo run -p migration -- up -n 1

migrate-down: ## Rollback all applied migrations
	cargo run -p migration -- down

migrate-rollback: ## Rollback only the last applied migration
	cargo run -p migration -- down -n 1

## ── Cleanup ──────────────────────────────────────────────────

clean: ## Remove build artifacts
	cargo clean

## ── Help ─────────────────────────────────────────────────────

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help
