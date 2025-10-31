pub use sea_orm_migration::prelude::*;

mod m20251031_154708_create_user_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251031_154708_create_user_schema::Migration),
        ]
    }
}
