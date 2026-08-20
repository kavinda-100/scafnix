use colored::*;

use crate::config::{package_manager::PackageManager, project::ProjectConfig};

/// Prints the project creation summary and next steps.
pub fn print_last_message(config: &ProjectConfig) {
    let package_manager = match config.package_manager {
        PackageManager::Pnpm => "pnpm",
        PackageManager::Bun => "bun",
    };

    println!(
        "{} Project {} created successfully!",
        "✓".green().bold(),
        config.name.green().bold()
    );
    println!(
        "  {} {}",
        "Location:".bright_black(),
        config.destination.display().to_string().bright_black()
    );
    println!();
    println!("{}", "Next steps:".bold());
    println!(
        "  {}",
        format!("cd {}", config.destination.display()).cyan()
    );

    if !config.install_dependencies {
        println!("  {}", format!("{} install", package_manager).cyan());
    }

    println!("  {}", format!("{} run build", package_manager).cyan());
    println!("  {}", format!("{} run dev", package_manager).cyan());

    if config.initialize_git {
        println!();
        println!("{}", "Git repository initialized.".bright_black());
    }
}
