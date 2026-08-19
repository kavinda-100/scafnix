use include_dir::{Dir, include_dir};

use crate::{
    config::project::ProjectConfig,
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static CONFIG_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/packages/config");

pub fn generate_config_package(
    config: &ProjectConfig,
    context: &TemplateContext,
) -> anyhow::Result<()> {
    let destination = config.destination.join("packages").join("config");

    extract_dir(&CONFIG_TEMPLATE, &destination, context)?;

    Ok(())
}
