use serde::{Deserialize, Serialize};

/// Azure DevOps Work Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    pub id: u64,
    pub rev: u64,
    pub fields: WorkItemFields,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItemFields {
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub work_item_type: Option<String>,
    #[serde(rename = "System.CreatedDate")]
    pub created_date: Option<String>,
    #[serde(rename = "System.ChangedDate")]
    pub changed_date: Option<String>,
    #[serde(rename = "System.CreatedBy")]
    pub created_by: Option<String>,
    #[serde(rename = "System.AssignedTo")]
    pub assigned_to: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Scheduling.Effort")]
    pub effort: Option<f64>,
    #[serde(rename = "System.IterationPath")]
    pub iteration_path: Option<String>,
}

/// Azure DevOps Project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub state: String,
}

/// Azure DevOps Sprint/Iteration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    pub id: String,
    pub name: String,
    pub path: String,
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub attributes: SprintAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintAttributes {
    pub start_date: Option<String>,
    pub finish_date: Option<String>,
    pub time_frame: Option<String>,
}

/// Azure DevOps Build
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
    pub build_number: String,
    pub status: String,
    pub result: Option<String>,
    pub definition: BuildDefinitionRef,
    pub queue_time: String,
    pub start_time: Option<String>,
    pub finish_time: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildDefinitionRef {
    pub id: u64,
    pub name: String,
    pub url: String,
}

/// Azure DevOps Release
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub created_on: String,
    pub modified_on: String,
    pub environments: Vec<ReleaseEnvironment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseEnvironment {
    pub id: u64,
    pub name: String,
    pub status: String,
}

/// Azure DevOps Repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub url: String,
    pub remote_url: Option<String>,
    pub project: Option<ProjectRef>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRef {
    pub id: String,
    pub name: String,
}

/// Azure DevOps Pull Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub pull_request_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_by: IdentityRef,
    pub creation_date: String,
    pub source_branch: String,
    pub target_branch: String,
    pub merge_status: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRef {
    pub display_name: Option<String>,
    pub unique_name: Option<String>,
    pub url: String,
}

/// Azure DevOps Board/Task Board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub url: String,
    pub columns: Vec<BoardColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardColumn {
    pub id: String,
    pub name: String,
    pub order: u64,
}

/// Webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub event_type: Option<String>,
    pub resource: Option<serde_json::Value>,
}
