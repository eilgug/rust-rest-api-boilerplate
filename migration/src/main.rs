use chrono::Local;
use sea_orm_migration::prelude::*;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Intercept our custom `new <name>` subcommand.
    if args.get(1).map(|s| s.as_str()) == Some("new") {
        match args.get(2) {
            Some(name) => {
                if let Err(e) = create_migration(name) {
                    eprintln!("Error creating migration: {e}");
                    std::process::exit(1);
                }
            }
            None => {
                eprintln!("Usage: cargo run -p migration -- new <migration_name>");
                std::process::exit(1);
            }
        }
        return;
    }

    // All other subcommands are handled by the standard SeaORM CLI.
    cli::run_cli(migration::Migrator).await;
}

/// Creates a new migration file in `migration/src/` with the format:
/// `YYYYMMDD_SERIAL_name.rs`, where SERIAL is zero-padded and sequential
/// per date (e.g. `000001`, `000002`, …).
fn create_migration(name: &str) -> std::io::Result<()> {
    // Sanitise user input: lowercase, spaces → underscores.
    let name = name.to_lowercase().replace(' ', "_");

    let date = Local::now().format("%Y%m%d").to_string();

    // Resolve the `migration/src/` directory relative to the workspace root.
    // When running via `cargo run -p migration`, the CWD is the workspace root.
    let src_dir = PathBuf::from("migration/src");

    // Determine the next sequential number for today by scanning existing files.
    let serial = next_serial(&src_dir, &date)?;

    let file_stem = format!("m{date}_{serial:0>6}_{name}");
    let file_name = format!("{file_stem}.rs");
    let file_path = src_dir.join(&file_name);

    if file_path.exists() {
        eprintln!("Migration file already exists: {}", file_path.display());
        std::process::exit(1);
    }

    let template = build_template(&file_stem);
    fs::write(&file_path, template)?;

    println!("Created migration: {}", file_path.display());
    println!(
        "\nNext steps:\n  1. Edit `{}` to implement `up` and `down`.\n  2. Register it in `migration/src/lib.rs`.",
        file_path.display()
    );

    Ok(())
}

/// Scans `src_dir` for files whose names start with `date_` and returns the
/// next integer serial (1-based).
fn next_serial(src_dir: &PathBuf, date: &str) -> std::io::Result<u32> {
    let prefix = format!("m{date}_");
    let mut max_serial: u32 = 0;

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if !file_name.starts_with(&prefix) {
            continue;
        }

        // File name pattern: YYYYMMDD_SERIAL_*.rs  — extract SERIAL part.
        if let Some(rest) = file_name.strip_prefix(&prefix) {
            // rest starts with the serial digits followed by '_'
            let serial_str: &str = rest.split('_').next().unwrap_or("0");
            if let Ok(n) = serial_str.parse::<u32>() {
                if n > max_serial {
                    max_serial = n;
                }
            }
        }
    }

    Ok(max_serial + 1)
}

/// Generates the boilerplate content for a new migration file.
fn build_template(file_stem: &str) -> String {
    // Derive a human-friendly struct name from the file stem by title-casing
    // each underscore-separated word (e.g. `20250227_000001_create_users` →
    // `M20250227000001CreateUsers`).
    let struct_name: String = {
        let mut s = String::from("M");
        for part in file_stem.split('_') {
            let mut chars = part.chars();
            match chars.next() {
                None => {}
                Some(first) => {
                    s.extend(first.to_uppercase());
                    s.push_str(chars.as_str());
                }
            }
        }
        s
    };

    format!(
        r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        // TODO: implement the migration
        let _ = manager;
        Ok(())
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        // TODO: implement the rollback
        let _ = manager;
        Ok(())
    }}
}}

// Rename this enum to match your table and add the relevant columns/variants.
#[derive(DeriveIden)]
enum {struct_name} {{
    Table,
    Id,
}}
"#
    )
}
