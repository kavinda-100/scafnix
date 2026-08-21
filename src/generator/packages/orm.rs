use include_dir::{Dir, include_dir};

use crate::{
    config::{orm_provider::OrmProvider, project::ProjectConfig},
    template::{extractor::extract_dir, renderer::TemplateContext},
};

static DATABASE_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/packages/database");
static PRISMA_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/orm-providers/prisma");
static DRIZZLE_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/orm-providers/drizzle");

pub fn generate_database_orm_package(
    config: &ProjectConfig,
    context: &TemplateContext,
) -> anyhow::Result<()> {
    let destination = config.destination.join("packages").join("database");

    extract_dir(&DATABASE_TEMPLATE, &destination, context)?;

    match config.orm_provider {
        OrmProvider::Prisma => {
            extract_dir(&PRISMA_TEMPLATE, &destination, context)?;
        }
        OrmProvider::Drizzle => {
            extract_dir(&DRIZZLE_TEMPLATE, &destination, context)?;
        }
    }

    Ok(())
}
