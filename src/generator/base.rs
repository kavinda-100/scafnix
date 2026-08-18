use include_dir::{Dir, include_dir};

use crate::{
    config::project::ProjectConfig,
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static BASE_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/base");

pub fn generate_base(config: &ProjectConfig, context: &TemplateContext) -> anyhow::Result<()> {
    extract_dir(&BASE_TEMPLATE, &config.destination, context)?;

    Ok(())
}
