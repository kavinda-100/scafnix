use include_dir::{Dir, include_dir};

use crate::{
    config::project::ProjectConfig,
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static BASE_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/base");

pub fn generate_project(config: &ProjectConfig) -> anyhow::Result<()> {
    let context = TemplateContext {
        project_name: config.name.clone(),
    };

    extract_dir(&BASE_TEMPLATE, &config.destination, &context)?;

    Ok(())
}
