use std::path::PathBuf;

use inquire::{Confirm, Select, Text};

use crate::config::{
    api_framework::ApiFramework, database_provider::DatabaseProvider,
    package_manager::PackageManager, project::ProjectConfig,
};

pub fn collect_project_config() -> anyhow::Result<ProjectConfig> {
    let project_name = Text::new("Project name:").with_default("my-app").prompt()?;

    let package_manager = Select::new(
        "Package manager:",
        vec![PackageManager::Pnpm, PackageManager::Bun],
    )
    .prompt()?;

    let api_framework = Select::new("API framework:", vec![ApiFramework::Express]).prompt()?;

    let database_provider = Select::new(
        "Database provider:",
        vec![DatabaseProvider::Prisma, DatabaseProvider::Drizzle],
    )
    .prompt()?;

    let install_dependencies = Confirm::new("Install dependencies?")
        .with_default(true)
        .prompt()?;

    let initialize_git = Confirm::new("Initialize git repository?")
        .with_default(true)
        .prompt()?;

    Ok(ProjectConfig {
        name: project_name.clone(),
        destination: PathBuf::from(&project_name),
        package_manager,
        api_framework,
        database_provider,
        install_dependencies,
        initialize_git,
    })
}
