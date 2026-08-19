use crate::{
    config::project::ProjectConfig,
    generator::{
        apps::api_framework::generate_apps_api_framework,
        base::generate_base,
        package_manager::generate_package_manager,
        packages::{
            config::generate_config_package, databse::generate_database_package,
            schema::generate_schema_package,
        },
    },
    template::renderer::TemplateContext,
};

pub fn generate_project(config: &ProjectConfig) -> anyhow::Result<()> {
    let context = TemplateContext {
        project_name: config.name.clone(),
    };

    generate_base(config, &context)?;

    generate_package_manager(config, &context)?;

    generate_config_package(config, &context)?;

    generate_schema_package(config, &context)?;

    generate_database_package(config, &context)?;

    generate_apps_api_framework(config, &context)?;

    Ok(())
}
