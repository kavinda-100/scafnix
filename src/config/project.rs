use std::path::PathBuf;

use super::package_manager::PackageManager;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub destination: PathBuf,
    pub package_manager: PackageManager,
}
