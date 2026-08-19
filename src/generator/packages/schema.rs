use include_dir::{Dir, include_dir};

use crate::{
    config::project::ProjectConfig,
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static SCHEMA_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/packages/schema");

pub fn generate_schema_package(
    config: &ProjectConfig,
    context: &TemplateContext,
) -> anyhow::Result<()> {
    let destination = config.destination.join("packages").join("schema");

    extract_dir(&SCHEMA_TEMPLATE, &destination, context)?;

    Ok(())
}
