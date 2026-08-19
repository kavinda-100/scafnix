use std::{env, path::PathBuf};

mod config;
mod generator;
mod template;

use config::{
    api_framework::ApiFramework, database_provider::DatabaseProvider,
    package_manager::PackageManager, project::ProjectConfig,
};

use generator::project::generate_project;

fn main() -> anyhow::Result<()> {
    let project_name = env::args().nth(1).unwrap_or_else(|| "demo".to_string());

    let config = ProjectConfig {
        name: project_name.clone(),
        destination: PathBuf::from(&project_name),
        package_manager: PackageManager::Pnpm,
        api_framework: ApiFramework::Express,
        database_provider: DatabaseProvider::Prisma,
    };

    generate_project(&config)?;

    println!("Created {}", config.destination.display());

    Ok(())
}
