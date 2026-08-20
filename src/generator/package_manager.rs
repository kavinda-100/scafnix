use include_dir::{Dir, include_dir};

use crate::{
    config::{package_manager::PackageManager, project::ProjectConfig},
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static PNPM_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/package-manager/pnpm");

pub fn generate_package_manager(
    config: &ProjectConfig,
    context: &TemplateContext,
) -> anyhow::Result<()> {
    match config.package_manager {
        PackageManager::Pnpm => {
            extract_dir(&PNPM_TEMPLATE, &config.destination, context)?;
        }

        PackageManager::Bun => {
            // Bun no need a workspace file, so we don't need to generate anything for it.
        }
    }

    Ok(())
}
