mod cli;
mod config;
mod generator;
mod process;
mod template;

use generator::project::generate_project;

use crate::cli::prompts::collect_project_config;

fn main() -> anyhow::Result<()> {
    let config = collect_project_config()?;

    generate_project(&config)?;

    println!("Created {}", config.destination.display());

    Ok(())
}
