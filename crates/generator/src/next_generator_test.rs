#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn get_template_path(template_name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("templates")
            .join("next")
            .join(template_name)
    }

    #[test]
    fn test_page_template_renders() {
        let template_path = get_template_path("page.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "page.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{entity_name}}"), "page template should have entity_name placeholder");
        assert!(content.contains("Metadata"), "page template should use Next.js Metadata");
        assert!(content.contains("export default function"), "page template should have default export");
        assert!(content.contains("container mx-auto"), "page template should use Tailwind classes");
    }

    #[test]
    fn test_hook_template_renders() {
        let template_path = get_template_path("hook.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "hook.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{entity_name}}"), "hook template should have entity_name placeholder");
        assert!(content.contains("useState"), "hook template should use React useState");
        assert!(content.contains("useEffect"), "hook template should use React useEffect");
        assert!(content.contains("use{{entity_name}}List"), "hook template should export list hook");
        assert!(content.contains("use{{entity_name}}ById"), "hook template should export by-id hook");
    }

    #[test]
    fn test_api_route_template_renders() {
        let template_path = get_template_path("api_route.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "api_route.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{entity_name}}"), "api_route template should have entity_name placeholder");
        assert!(content.contains("NextRequest"), "api_route template should use NextRequest");
        assert!(content.contains("NextResponse"), "api_route template should use NextResponse");
        assert!(content.contains("export async function GET"), "api_route template should export GET handler");
        assert!(content.contains("export async function POST"), "api_route template should export POST handler");
    }

    #[test]
    fn test_types_template_renders() {
        let template_path = get_template_path("types.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "types.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{entity_name}}"), "types template should have entity_name placeholder");
        assert!(content.contains("export interface"), "types template should export TypeScript interfaces");
        assert!(content.contains("Create{{entity_name}}DTO"), "types template should have create DTO");
        assert!(content.contains("Update{{entity_name}}DTO"), "types template should have update DTO");
    }

    #[test]
    fn test_component_template_renders() {
        let template_path = get_template_path("component.hbs");
        let content = std::fs::read_to_string(&template_path);
        assert!(content.is_ok(), "component.hbs should exist and be readable");

        let content = content.unwrap();
        assert!(content.contains("{{entity_name}}"), "component template should have entity_name placeholder");
        assert!(content.contains("use{{entity_name}}List"), "component template should use the list hook");
        assert!(content.contains("{{entity_name}}Card"), "component template should render card component");
        assert!(content.contains("'use client'"), "component template should be a client component");
    }

    #[test]
    fn test_all_next_templates_exist() {
        let templates = vec![
            "page.hbs",
            "hook.hbs",
            "api_route.hbs",
            "types.hbs",
            "component.hbs",
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