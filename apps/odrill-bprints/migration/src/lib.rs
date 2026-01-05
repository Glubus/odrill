#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20260102_151644_packages;
mod m20260102_152004_versions;
mod m20260103_120000_api_keys;
mod m20260103_120001_api_historic;
mod m20260103_130000_templates;
mod m20260103_140000_templates_version;
mod m20260103_220000_add_downloads;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260102_151644_packages::Migration),
            Box::new(m20260102_152004_versions::Migration),
            Box::new(m20260103_120000_api_keys::Migration),
            Box::new(m20260103_120001_api_historic::Migration),
            Box::new(m20260103_130000_templates::Migration),
            Box::new(m20260103_140000_templates_version::Migration),
            Box::new(m20260103_220000_add_downloads::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
