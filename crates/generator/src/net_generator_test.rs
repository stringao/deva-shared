#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn get_template_path(template_name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
            .join("net")
            .join(template_name)
    }

    #[test]
    fn test_entity_template_renders() {
        let template_path = get_template_path("entity.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "entity.hbs should exist and be readable");

        let content = content.unwrap();
        // Should contain namespace placeholder
        assert!(content.contains("{{namespace}}"), "entity template should have namespace placeholder");
        // Should have proper C# class structure
        assert!(content.contains("public class"), "entity template should have C# class declaration");
    }

    #[test]
    fn test_handler_template_renders() {
        let template_path = get_template_path("handler.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "handler.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{namespace}}"), "handler template should have namespace placeholder");
        assert!(content.contains("IRequestHandler"), "handler template should use MediatR IRequestHandler");
    }

    #[test]
    fn test_endpoint_template_renders() {
        let template_path = get_template_path("endpoint.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "endpoint.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{namespace}}"), "endpoint template should have namespace placeholder");
        assert!(content.contains("ICarterModule"), "endpoint template should implement Carter interface");
    }

    #[test]
    fn test_validator_template_renders() {
        let template_path = get_template_path("validator.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "validator.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{namespace}}"), "validator template should have namespace placeholder");
        assert!(content.contains("AbstractValidator"), "validator template should use FluentValidation");
    }

    #[test]
    fn test_dbset_template_renders() {
        let template_path = get_template_path("dbset.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "dbset.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{namespace}}"), "dbset template should have namespace placeholder");
        assert!(content.contains("DbSet"), "dbset template should use EF Core DbSet");
    }

    #[test]
    fn test_all_templates_exist() {
        let templates = vec![
            "entity.hbs",
            "handler.hbs",
            "endpoint.hbs",
            "validator.hbs",
            "dbset.hbs",
        ];

        for template in templates {
            let path = get_template_path(template);
            assert!(path.exists(), "{} should exist at {:?}", template, path);
            let content = std::fs::read_to_string(&path);
            assert!(content.is_ok(), "{} should be readable", template);
            assert!(!content.unwrap().is_empty(), "{} should not be empty", template);
        }
    }
}