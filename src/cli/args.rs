use crate::config::{
    api_framework::ApiFramework, orm_provider::OrmProvider, package_manager::PackageManager,
};
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "scafnix", version, about = "Generate a TypeScript monorepo")]
pub struct Cli {
    /// Name of the project
    pub project_name: Option<String>,

    /// Package manager to use
    #[arg(short = 'p', long = "package-manager")]
    pub package_manager: Option<CliPackageManager>,

    /// API framework to use
    #[arg(short = 'a', long = "api")]
    pub api_framework: Option<CliApiFramework>,

    /// ORM provider to use
    #[arg(short = 'o', long = "orm")]
    pub orm_provider: Option<CliOrmProvider>,

    /// Skip dependency installation
    #[arg(long)]
    pub no_install: bool,

    /// Skip Git initialization
    #[arg(long)]
    pub no_git: bool,

    /// Use defaults for all unspecified options
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliPackageManager {
    Pnpm,
    Bun,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliApiFramework {
    Express,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliOrmProvider {
    Prisma,
    Drizzle,
}

impl From<CliPackageManager> for PackageManager {
    fn from(value: CliPackageManager) -> Self {
        match value {
            CliPackageManager::Pnpm => Self::Pnpm,
            CliPackageManager::Bun => Self::Bun,
        }
    }
}

impl From<CliApiFramework> for ApiFramework {
    fn from(value: CliApiFramework) -> Self {
        match value {
            CliApiFramework::Express => Self::Express,
        }
    }
}

impl From<CliOrmProvider> for OrmProvider {
    fn from(value: CliOrmProvider) -> Self {
        match value {
            CliOrmProvider::Prisma => Self::Prisma,
            CliOrmProvider::Drizzle => Self::Drizzle,
        }
    }
}
