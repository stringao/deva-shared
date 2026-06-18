use std::path::Path;

#[test]
fn test_workspace_has_eight_crates() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();

    let expected_crates = vec![
        "crates/core",
        "crates/cli",
        "crates/server",
        "crates/mcp",
        "crates/github",
        "crates/azure_devops",
        "crates/telegram",
        "crates/generator",
    ];

    for crate_path in expected_crates {
        let full_path = workspace_root.join(crate_path).join("Cargo.toml");
        assert!(
            full_path.exists(),
            "Expected crate at {}",
            full_path.display()
        );
    }
}

#[test]
fn test_all_crates_have_lib_rs() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();

    let expected_crates = vec![
        "crates/core",
        "crates/cli",
        "crates/server",
        "crates/mcp",
        "crates/github",
        "crates/azure_devops",
        "crates/telegram",
        "crates/generator",
    ];

    for crate_path in expected_crates {
        let lib_path = workspace_root.join(crate_path).join("src").join("lib.rs");
        assert!(
            lib_path.exists(),
            "Expected lib.rs at {}",
            lib_path.display()
        );
    }
}

#[test]
fn test_all_crates_define_error_module() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();

    let expected_crates = vec![
        "crates/core",
        "crates/cli",
        "crates/server",
        "crates/mcp",
        "crates/github",
        "crates/azure_devops",
        "crates/telegram",
        "crates/generator",
    ];

    for crate_path in expected_crates {
        let error_path = workspace_root.join(crate_path).join("src").join("error.rs");
        assert!(
            error_path.exists(),
            "Expected error.rs at {}",
            error_path.display()
        );
    }
}

#[test]
fn test_workspace_root_cargo_toml_exists() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let cargo_toml = workspace_root.join("Cargo.toml");
    assert!(cargo_toml.exists(), "Workspace Cargo.toml should exist");
}

#[test]
fn test_rustfmt_toml_exists() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let rustfmt_toml = workspace_root.join("rustfmt.toml");
    assert!(rustfmt_toml.exists(), "rustfmt.toml should exist");
}

#[test]
fn test_dockerfile_exists() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let dockerfile = workspace_root.join("Dockerfile");
    assert!(dockerfile.exists(), "Dockerfile should exist");
}

#[test]
fn test_docker_compose_yml_exists() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let compose_file = workspace_root.join("docker-compose.yml");
    assert!(compose_file.exists(), "docker-compose.yml should exist");
}

#[test]
fn test_ci_workflow_exists() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    let ci_workflow = workspace_root.join(".github").join("workflows").join("ci.yml");
    assert!(ci_workflow.exists(), "CI workflow should exist at {}", ci_workflow.display());
}