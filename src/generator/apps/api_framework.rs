use include_dir::{Dir, include_dir};

use crate::{
    config::{api_framework::ApiFramework, project::ProjectConfig},
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static API_BASE_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/apps/api");
static EXPRESS_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/api-providers/express");

pub fn generate_apps_api_framework(
    config: &ProjectConfig,
    context: &TemplateContext,
) -> anyhow::Result<()> {
    let destination = config.destination.join("apps").join("api");

    extract_dir(&API_BASE_TEMPLATE, &destination, context)?;

    match config.api_framework {
        ApiFramework::Express => {
            extract_dir(&EXPRESS_TEMPLATE, &destination, context)?;
        }
    }

    Ok(())
}
