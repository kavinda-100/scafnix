use include_dir::{Dir, include_dir};
use std::{env, path::PathBuf};

mod template;

use template::{extractor::extract_dir, renderer::TemplateContext};

static BASE_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/base");

fn main() -> anyhow::Result<()> {
    let project_name = env::args().nth(1).unwrap_or_else(|| "demo".to_string());

    let destination = PathBuf::from(&project_name);

    let context = TemplateContext {
        project_name: project_name.clone(),
    };

    extract_dir(&BASE_TEMPLATE, &destination, &context)?;

    println!("Created {}", destination.display());

    Ok(())
}
