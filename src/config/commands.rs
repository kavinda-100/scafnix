use crate::config::{database_provider::DatabaseProvider, package_manager::PackageManager};

pub struct ProjectCommands {
    pub start: String,
    pub predev: String,
    pub dev: String,
    pub build: String,

    pub db_generate: String,
    pub db_push: String,
    pub db_studio: String,
    pub db_reset: String,
}

impl ProjectCommands {
    pub fn new(
        package_manager: &PackageManager,
        database_provider: &DatabaseProvider,
        project_name: &str,
    ) -> Self {
        let start = package_manager.start_command().to_string();
        let predev = package_manager.predev_command().to_string();
        let dev = package_manager.dev_command().to_string();
        let build = package_manager.build_command().to_string();

        let database_package = format!("@{project_name}/database");

        let (db_generate, db_push, db_studio, db_reset) = match (package_manager, database_provider)
        {
            (PackageManager::Pnpm, DatabaseProvider::Prisma) => (
                format!("pnpm --filter {database_package} db:generate"),
                format!("pnpm --filter {database_package} db:push"),
                format!("pnpm --filter {database_package} db:studio"),
                format!("pnpm --filter {database_package} db:reset"),
            ),

            (PackageManager::Bun, DatabaseProvider::Prisma) => (
                format!("bun --filter {database_package} db:generate"),
                format!("bun --filter {database_package} db:push"),
                format!("bun --filter {database_package} db:studio"),
                format!("bun --filter {database_package} db:reset"),
            ),

            (PackageManager::Pnpm, DatabaseProvider::Drizzle) => {
                todo!("Drizzle commands are not implemented yet")
            }

            (PackageManager::Bun, DatabaseProvider::Drizzle) => {
                todo!("Drizzle commands are not implemented yet")
            }
        };

        Self {
            start,
            predev,
            dev,
            build,
            db_generate,
            db_push,
            db_studio,
            db_reset,
        }
    }
}
