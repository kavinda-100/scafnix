use std::path::PathBuf;

use inquire::{Confirm, Select, Text};

use crate::config::{
    api_framework::ApiFramework, database_provider::DatabaseProvider,
    package_manager::PackageManager, project::ProjectConfig,
};

pub fn collect_project_config() -> anyhow::Result<ProjectConfig> {
    // -- prompts ---

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

    // -- convert to enums ---

    // let api_framework = match api_framework {
    //     "Express" => ApiFramework::Express,
    //     _ => unreachable!(),
    // };

    // let package_manager = match package_manager {
    //     "pnpm" => PackageManager::Pnpm,
    //     "bun" => PackageManager::Bun,
    //     _ => unreachable!(),
    // };

    // let database_provider = match database_provider {
    //     "Prisma" => DatabaseProvider::Prisma,
    //     "Drizzle" => DatabaseProvider::Drizzle,
    //     _ => unreachable!(),
    // };

    Ok(ProjectConfig {
        name: project_name.clone(),
        destination: PathBuf::from(&project_name),
        package_manager,
        api_framework,
        database_provider,
        install_dependencies: install_dependencies,
        initialize_git: initialize_git,
    })
}
