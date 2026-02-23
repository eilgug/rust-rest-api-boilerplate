pub use sea_orm_migration::prelude::*;

mod m20250223_000001_create_profiles_table;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20250223_000001_create_profiles_table::Migration,
        )]
    }
}
