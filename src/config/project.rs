use std::path::PathBuf;

use super::api_framework::ApiFramework;
use super::orm_provider::OrmProvider;
use super::package_manager::PackageManager;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub destination: PathBuf,
    pub package_manager: PackageManager,
    pub api_framework: ApiFramework,
    pub orm_provider: OrmProvider,
    pub install_dependencies: bool,
    pub initialize_git: bool,
}
