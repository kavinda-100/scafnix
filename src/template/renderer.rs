pub struct TemplateContext {
    pub project_name: String,
}

pub fn render(content: &str, context: &TemplateContext) -> String {
    content.replace("{{project_name}}", &context.project_name)
}
