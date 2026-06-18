use serde::{Deserialize, Serialize};

/// GitHub Repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub description: Option<String>,
    pub private: bool,
    pub default_branch: String,
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub html_url: String,
}

/// GitHub Pull Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub mergeable: Option<bool>,
    pub draft: bool,
    pub head: GitRef,
    pub base: GitRef,
    pub requested_reviewers: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRef {
    pub ref_: String,
    pub sha: String,
    pub repo: Repository,
}

/// GitHub Issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub user: User,
    pub labels: Vec<Label>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

/// GitHub Branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
    pub protected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub html_url: String,
    pub message: String,
    pub author: Option<CommitAuthor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

/// GitHub Commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub sha: String,
    pub commit: Commit,
    pub html_url: String,
}

/// GitHub Webhook Events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum PullRequestEvent {
    #[serde(rename = "opened")]
    Opened { pull_request: PullRequest, repository: Repository },
    #[serde(rename = "closed")]
    Closed { pull_request: PullRequest, merged: bool, repository: Repository },
    #[serde(rename = "synchronize")]
    Synchronize { pull_request: PullRequest, repository: Repository },
    #[serde(rename = "review_requested")]
    ReviewRequested { pull_request: PullRequest, requested_reviewer: User, repository: Repository },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum IssueEvent {
    #[serde(rename = "opened")]
    Opened { issue: Issue, repository: Repository },
    #[serde(rename = "closed")]
    Closed { issue: Issue, repository: Repository },
    #[serde(rename = "reopened")]
    Reopened { issue: Issue, repository: Repository },
    #[serde(rename = "labeled")]
    Labeled { issue: Issue, label: Label, repository: Repository },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushEvent {
    pub ref_: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub commits: Vec<Commit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

/// Webhook payload wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub action: Option<String>,
    pub repository: Option<Repository>,
    pub pull_request: Option<PullRequest>,
    pub issue: Option<Issue>,
    pub ref_: Option<String>,
}
