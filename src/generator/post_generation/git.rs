use crate::{config::project::ProjectConfig, process::command::run_command};

pub fn initialize_git(config: &ProjectConfig) -> anyhow::Result<()> {
    run_command("git", &["init", "-b", "main"], &config.destination)
}
