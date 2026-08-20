pub struct TemplateContext {
    pub project_name: String,

    pub start_command: String,
    pub predev_command: String,
    pub dev_command: String,
    pub build_command: String,

    pub db_generate_command: String,
    pub db_push_command: String,
    pub db_studio_command: String,
    pub db_reset_command: String,
}

pub fn render(content: &str, context: &TemplateContext) -> String {
    content
        .replace("{{project_name}}", &context.project_name)
        .replace("{{start_command}}", &context.start_command)
        .replace("{{predev_command}}", &context.predev_command)
        .replace("{{dev_command}}", &context.dev_command)
        .replace("{{build_command}}", &context.build_command)
        .replace("{{db_generate_command}}", &context.db_generate_command)
        .replace("{{db_push_command}}", &context.db_push_command)
        .replace("{{db_studio_command}}", &context.db_studio_command)
        .replace("{{db_reset_command}}", &context.db_reset_command)
}
