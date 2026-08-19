use crate::{config::project::ProjectConfig, process::command::run_command};

pub fn install_dependencies(config: &ProjectConfig) -> anyhow::Result<()> {
    let package_manager = &config.package_manager;

    run_command(
        package_manager.executable(),
        package_manager.install_args(),
        &config.destination,
    )
}
