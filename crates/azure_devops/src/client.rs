use crate::error::AzureError;
use crate::models::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Azure DevOps API Client
#[derive(Clone)]
pub struct AzureDevOpsClient {
    client: reqwest::Client,
    organization: String,
    project: String,
    token: String,
}

impl AzureDevOpsClient {
    /// Create a new Azure DevOps client
    pub fn new(token: impl Into<String>, organization: impl Into<String>, project: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.into(),
            organization: organization.into(),
            project: project.into(),
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self, AzureError> {
        let token = std::env::var("AZURE_DEVOPS_TOKEN")
            .map_err(|_| AzureError::AuthError("AZURE_DEVOPS_TOKEN not set".into()))?;
        let organization = std::env::var("AZURE_DEVOPS_ORG")
            .map_err(|_| AzureError::AuthError("AZURE_DEVOPS_ORG not set".into()))?;
        let project = std::env::var("AZURE_DEVOPS_PROJECT")
            .map_err(|_| AzureError::AuthError("AZURE_DEVOPS_PROJECT not set".into()))?;

        Ok(Self::new(token, organization, project))
    }

    fn auth_header(&self) -> String {
        use base64::Engine;
        let creds = base64::engine::general_purpose::STANDARD
            .encode(format!(":{}", self.token));
        format!("Basic {}", creds)
    }

    fn api_url(&self, path: &str) -> String {
        format!(
            "https://dev.azure.com/{}/{}/_apis{}?api-version=7.0",
            self.organization,
            self.project,
            path
        )
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, AzureError> {
        let url = self.api_url(path);
        let resp = self.client.get(&url)
            .header("Authorization", self.auth_header())
            .header("User-Agent", "deva-azure-devops")
            .send()
            .await
            .map_err(|e| AzureError::ApiError(e.to_string()))?;

        if resp.status() == 401 {
            return Err(AzureError::AuthError("Unauthorized".into()));
        }
        if resp.status() == 404 {
            return Err(AzureError::NotFound(path.into()));
        }

        resp.json().await.map_err(|e| AzureError::ApiError(e.to_string()))
    }

    async fn post<T: serde::de::DeserializeOwned>(&self, path: &str, body: Value) -> Result<T, AzureError> {
        let url = self.api_url(path);
        let resp = self.client.post(&url)
            .header("Authorization", self.auth_header())
            .header("User-Agent", "deva-azure-devops")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AzureError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(AzureError::ApiError(format!("{:?}", err)));
        }

        resp.json().await.map_err(|e| AzureError::ApiError(e.to_string()))
    }

    async fn patch<T: serde::de::DeserializeOwned>(&self, path: &str, body: Value) -> Result<T, AzureError> {
        let url = self.api_url(path);
        let resp = self.client.patch(&url)
            .header("Authorization", self.auth_header())
            .header("User-Agent", "deva-azure-devops")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AzureError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(AzureError::ApiError(format!("{:?}", err)));
        }

        resp.json().await.map_err(|e| AzureError::ApiError(e.to_string()))
    }

    async fn delete(&self, path: &str) -> Result<(), AzureError> {
        let url = self.api_url(path);
        let resp = self.client.delete(&url)
            .header("Authorization", self.auth_header())
            .header("User-Agent", "deva-azure-devops")
            .send()
            .await
            .map_err(|e| AzureError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(AzureError::ApiError(format!("{:?}", err)));
        }

        Ok(())
    }

    // ============ Work Items ============

    /// List work items
    pub async fn list_work_items(&self, work_item_type: Option<&str>, state: Option<&str>) -> Result<Vec<WorkItem>, AzureError> {
        let mut path = "/wit/workitems".to_string();
        let mut query_params = Vec::new();

        if let Some(t) = work_item_type {
            query_params.push(format!("$filter=Work Item Type eq '{}'", t));
        }
        if let Some(s) = state {
            if query_params.is_empty() {
                query_params.push(format!("$filter=State eq '{}'", s));
            } else {
                query_params.push(format!("and State eq '{}'", s));
            }
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("")));
        }

        #[derive(Deserialize)]
        struct WorkItemResponse { value: Vec<WorkItem> }

        let result: WorkItemResponse = self.get(&path).await?;
        Ok(result.value)
    }

    /// Get a work item
    pub async fn get_work_item(&self, id: u64) -> Result<WorkItem, AzureError> {
        let path = format!("/wit/workitems/{}", id);
        self.get(&path).await
    }

    /// Create a work item
    pub async fn create_work_item(&self, work_item_type: &str, title: &str) -> Result<WorkItem, AzureError> {
        let path = format!("/wit/workitems?{}={}", "type", work_item_type);
        let body = serde_json::json!([
            { "op": "add", "path": "/fields/System.Title", "value": title }
        ]);
        self.post(&path, body).await
    }

    /// Update a work item
    pub async fn update_work_item(&self, id: u64, changes: Vec<( &str, &str)>) -> Result<WorkItem, AzureError> {
        let path = format!("/wit/workitems/{}", id);
        let ops: Vec<_> = changes.into_iter()
            .map(|(field, value)| serde_json::json!({
                "op": "add",
                "path": format!("/fields/{}", field),
                "value": value
            }))
            .collect();
        self.patch(&path, serde_json::json!(ops)).await
    }

    /// Delete a work item
    pub async fn delete_work_item(&self, id: u64) -> Result<(), AzureError> {
        let path = format!("/wit/workitems/{}", id);
        self.delete(&path).await
    }

    // ============ Sprints ============

    /// List sprints/iterations
    pub async fn list_sprints(&self) -> Result<Vec<Sprint>, AzureError> {
        #[derive(Deserialize)]
        struct SprintResponse { value: Vec<Sprint> }

        let result: SprintResponse = self.get("/wit/iterations").await?;
        Ok(result.value)
    }

    /// Get sprint details
    pub async fn get_sprint(&self, sprint_id: &str) -> Result<Sprint, AzureError> {
        let path = format!("/wit/iterations/{}", sprint_id);
        self.get(&path).await
    }

    // ============ Builds ============

    /// List build pipelines
    pub async fn list_builds(&self) -> Result<Vec<Build>, AzureError> {
        #[derive(Deserialize)]
        struct BuildResponse { value: Vec<Build> }

        let result: BuildResponse = self.get("/build/builds").await?;
        Ok(result.value)
    }

    /// Trigger a build
    pub async fn trigger_build(&self, definition_id: u64, branch: Option<&str>) -> Result<Build, AzureError> {
        let mut body = serde_json::json!({
            "definition": { "id": definition_id }
        });
        if let Some(b) = branch {
            body["parameters"] = serde_json::json!({ "SourceBranch": b });
        }
        self.post("/build/builds", body).await
    }

    // ============ Releases ============

    /// List releases
    pub async fn list_releases(&self) -> Result<Vec<Release>, AzureError> {
        #[derive(Deserialize)]
        struct ReleaseResponse { value: Vec<Release> }

        let result: ReleaseResponse = self.get("/release/releases").await?;
        Ok(result.value)
    }

    /// Create a release
    pub async fn create_release(&self, definition_id: u64, description: Option<&str>) -> Result<Release, AzureError> {
        let body = serde_json::json!({
            "definitionId": definition_id,
            "description": description.unwrap_or("")
        });
        self.post("/release/releases", body).await
    }

    // ============ Repositories ============

    /// List Git repositories
    pub async fn list_repos(&self) -> Result<Vec<Repository>, AzureError> {
        #[derive(Deserialize)]
        struct RepoResponse { value: Vec<Repository> }

        let result: RepoResponse = self.get("/git/repositories").await?;
        Ok(result.value)
    }

    // ============ Pull Requests ============

    /// List pull requests
    pub async fn list_pull_requests(&self, repo_id: Option<&str>, status: Option<&str>) -> Result<Vec<PullRequest>, AzureError> {
        let mut path = "/git/pullrequests".to_string();
        let mut params = Vec::new();

        if let Some(rid) = repo_id {
            params.push(format!("repositoryId={}", rid));
        }
        if let Some(s) = status {
            params.push(format!("searchCriteria.status={}", s));
        }

        if !params.is_empty() {
            path.push_str(&format!("?{}", params.join("&")));
        }

        #[derive(Deserialize)]
        struct PRResponse { value: Vec<PullRequest> }

        let result: PRResponse = self.get(&path).await?;
        Ok(result.value)
    }

    // ============ Boards ============

    /// Get task board
    pub async fn get_board(&self, board_name: &str) -> Result<Board, AzureError> {
        let path = format!("/wit/boards/{}", board_name);

        #[derive(Deserialize)]
        struct BoardResponse {
            id: String,
            name: String,
            url: String,
        }

        let result: BoardResponse = self.get(&path).await?;
        Ok(Board {
            id: result.id,
            name: result.name,
            url: result.url,
            columns: vec![],
        })
    }
}
