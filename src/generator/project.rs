use crate::{
    config::{commands::ProjectCommands, project::ProjectConfig},
    generator::{
        apps::api_framework::generate_apps_api_framework,
        base::generate_base,
        package_manager::generate_package_manager,
        packages::{
            config::generate_config_package, databse::generate_database_package,
            schema::generate_schema_package,
        },
        post_generation::{
            dependencies::{install_dependencies, upgrade_dependencies},
            git::initialize_git,
        },
    },
    template::renderer::TemplateContext,
    ui::seperater::print_separator,
};

pub fn generate_project(config: &ProjectConfig) -> anyhow::Result<()> {
    let commands = ProjectCommands::new(
        &config.package_manager,
        &config.database_provider,
        &config.name,
    );

    let context = TemplateContext {
        project_name: config.name.clone(),

        start_command: commands.start,
        predev_command: commands.predev,
        dev_command: commands.dev,
        build_command: commands.build,

        db_generate_command: commands.db_generate,
        db_push_command: commands.db_push,
        db_studio_command: commands.db_studio,
        db_reset_command: commands.db_reset,
    };

    // -- Generate project structure and files --

    generate_base(config, &context)?;

    generate_package_manager(config, &context)?;

    generate_config_package(config, &context)?;

    generate_schema_package(config, &context)?;

    generate_database_package(config, &context)?;

    generate_apps_api_framework(config, &context)?;

    // -- Post-generation tasks --

    // run before dependencies installation.
    if config.initialize_git {
        initialize_git(config)?;

        print_separator();
    }

    if config.install_dependencies {
        install_dependencies(config)?;
        upgrade_dependencies(config)?;
    }

    Ok(())
}
