use std::path::PathBuf;

use inquire::{Confirm, Select, Text};

use crate::{
    cli::args::Cli,
    config::{
        api_framework::ApiFramework, database_provider::DatabaseProvider,
        package_manager::PackageManager, project::ProjectConfig,
    },
};

pub fn collect_project_config(cli: Cli) -> anyhow::Result<ProjectConfig> {
    let project_name = match cli.project_name {
        Some(name) => name,

        None if cli.yes => "my-app".to_string(),

        None => Text::new("Project name:").with_default("my-app").prompt()?,
    };

    let package_manager = match cli.package_manager {
        Some(value) => value.into(),

        None if cli.yes => PackageManager::Pnpm,

        None => Select::new(
            "Package manager:",
            vec![PackageManager::Pnpm, PackageManager::Bun],
        )
        .prompt()?,
    };

    let api_framework = match cli.api_framework {
        Some(value) => value.into(),

        None if cli.yes => ApiFramework::Express,

        None => Select::new("API framework:", vec![ApiFramework::Express]).prompt()?,
    };

    let database_provider = match cli.database_provider {
        Some(value) => value.into(),

        None if cli.yes => DatabaseProvider::Prisma,

        None => Select::new(
            "Database provider:",
            vec![DatabaseProvider::Prisma, DatabaseProvider::Drizzle],
        )
        .prompt()?,
    };

    let install_dependencies = if cli.no_install {
        false
    } else if cli.yes {
        true
    } else {
        Confirm::new("Install dependencies?")
            .with_default(true)
            .prompt()?
    };

    let initialize_git = if cli.no_git {
        false
    } else if cli.yes {
        true
    } else {
        Confirm::new("Initialize git repository?")
            .with_default(true)
            .prompt()?
    };

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
