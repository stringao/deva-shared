use anyhow::Result;
use handlebars::Handlebars;
use std::path::{Path, PathBuf};
use serde_json::json;

fn chrono_lite_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Simple ISO-like date string from epoch
    let days = secs / 86400;
    let secs_in_day = secs % 86400;
    let hours = secs_in_day / 3600;
    let minutes = (secs_in_day % 3600) / 60;
    format!("1970+{}d {}h:{}m", days, hours, minutes)
}

pub struct Generator {
    templates: Handlebars<'static>,
}

impl Generator {
    pub fn new() -> Self {
        let mut templates = Handlebars::new();
        templates.set_strict_mode(true);
        Self { templates }
    }

    pub fn generate<P: AsRef<Path>>(
        &self,
        template_name: &str,
        data: &serde_json::Value,
        output_path: P,
    ) -> Result<()> {
        // Implementation
        Ok(())
    }

    pub fn generate_entity<P: AsRef<Path>>(&self, project_path: P, name: &str) -> Result<()> {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
            .join("net")
            .join("entity.hbs");

        let template_content = std::fs::read_to_string(&template_path)
            .map_err(|e| anyhow::anyhow!("Failed to read entity template: {}", e))?;

        let data = json!({
            "namespace": format!("App.Entities.{}", name),
            "entity_name": name,
            "generated_at": chrono_lite_date(),
            "properties": [],
        });

        let rendered = self.templates.render_template(&template_content, &data)
            .map_err(|e| anyhow::anyhow!("Template render error: {}", e))?;

        let output_path = project_path.as_ref().join(format!("{}.cs", name));
        std::fs::write(&output_path, rendered)
            .map_err(|e| anyhow::anyhow!("Failed to write entity: {}", e))?;

        Ok(())
    }

    pub fn generate_page<P: AsRef<Path>>(&self, project_path: P, name: &str) -> Result<()> {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
            .join("next")
            .join("page.hbs");

        let template_content = std::fs::read_to_string(&template_path)
            .map_err(|e| anyhow::anyhow!("Failed to read page template: {}", e))?;

        let data = json!({
            "entity_name": name,
        });

        let rendered = self.templates.render_template(&template_content, &data)
            .map_err(|e| anyhow::anyhow!("Template render error: {}", e))?;

        let output_path = project_path.as_ref().join(format!("{}.tsx", name));
        std::fs::write(&output_path, rendered)
            .map_err(|e| anyhow::anyhow!("Failed to write page: {}", e))?;

        Ok(())
    }

    pub fn generate_hook<P: AsRef<Path>>(&self, project_path: P, name: &str) -> Result<()> {
        let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
            .join("next")
            .join("hook.hbs");

        let template_content = std::fs::read_to_string(&template_path)
            .map_err(|e| anyhow::anyhow!("Failed to read hook template: {}", e))?;

        let data = json!({
            "entity_name": name,
        });

        let rendered = self.templates.render_template(&template_content, &data)
            .map_err(|e| anyhow::anyhow!("Template render error: {}", e))?;

        let output_path = project_path.as_ref().join(format!("use{}{}.ts", name, "s"));
        std::fs::write(&output_path, rendered)
            .map_err(|e| anyhow::anyhow!("Failed to write hook: {}", e))?;

        Ok(())
    }
}