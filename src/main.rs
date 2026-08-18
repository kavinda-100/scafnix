use std::{env, path::PathBuf};

mod config;
mod generator;
mod template;

use config::{package_manager::PackageManager, project::ProjectConfig};

use generator::project::generate_project;

fn main() -> anyhow::Result<()> {
    let project_name = env::args().nth(1).unwrap_or_else(|| "demo".to_string());

    let config = ProjectConfig {
        name: project_name.clone(),
        destination: PathBuf::from(&project_name),
        package_manager: PackageManager::Pnpm,
    };

    generate_project(&config)?;

    println!("Created {}", config.destination.display());

    Ok(())
}
