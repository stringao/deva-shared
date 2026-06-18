use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub project_type: ProjectType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    Fullstack,
    Backend,
    Frontend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScore {
    pub score: u8,
    pub issues: Vec<HealthIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthIssue {
    pub severity: Severity,
    pub message: String,
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}