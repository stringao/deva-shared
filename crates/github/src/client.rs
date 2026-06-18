use crate::error::GitHubError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// GitHub API Client using reqwest
#[derive(Clone)]
pub struct GitHubClient {
    client: reqwest::Client,
    token: String,
    owner: String,
    repo: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(token: impl Into<String>, owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.into(),
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self, GitHubError> {
        let token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| GitHubError::AuthError("GITHUB_TOKEN not set".into()))?;
        let owner = std::env::var("GITHUB_OWNER")
            .map_err(|_| GitHubError::AuthError("GITHUB_OWNER not set".into()))?;
        let repo = std::env::var("GITHUB_REPO")
            .map_err(|_| GitHubError::AuthError("GITHUB_REPO not set".into()))?;

        Ok(Self::new(token, owner, repo))
    }

    fn request(&self) -> reqwest::RequestBuilder {
        self.client
            .get(&format!("https://api.github.com/repos/{}/{}/", self.owner, self.repo))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
    }

    fn post_request(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .post(&format!("https://api.github.com/repos/{}/{}/{}", self.owner, self.repo, path))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
    }

    fn patch_request(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .patch(&format!("https://api.github.com/repos/{}/{}/{}", self.owner, self.repo, path))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
    }

    fn delete_request(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .delete(&format!("https://api.github.com/repos/{}/{}/{}", self.owner, self.repo, path))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
    }

    // ============ Repository ============

    /// Get repository info
    pub async fn get_repo(&self) -> Result<Value, GitHubError> {
        let resp = self.request()
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if resp.status() == 404 {
            return Err(GitHubError::NotFound(format!("{}/{}", self.owner, self.repo)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    // ============ Pull Requests ============

    /// List pull requests
    pub async fn list_prs(&self, state: Option<&str>) -> Result<Vec<Value>, GitHubError> {
        let mut url = format!("https://api.github.com/repos/{}/{}/pulls", self.owner, self.repo);
        if let Some(s) = state {
            url.push_str(&format!("?state={}", s));
        }

        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Get a specific PR
    pub async fn get_pr(&self, pr_number: u64) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}", self.owner, self.repo, pr_number);
        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if resp.status() == 404 {
            return Err(GitHubError::NotFound(format!("PR #{}", pr_number)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Create a pull request
    pub async fn create_pr(&self, title: &str, head: &str, base: &str, body: Option<&str>) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls", self.owner, self.repo);
        let mut body_map = serde_json::json!({
            "title": title,
            "head": head,
            "base": base
        });
        if let Some(b) = body {
            body_map["body"] = serde_json::json!(b);
        }

        let resp = self.post_request("pulls")
            .json(&body_map)
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(GitHubError::ApiError(format!("{:?}", err)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Merge a PR
    pub async fn merge_pr(&self, pr_number: u64, merge_method: Option<&str>) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}/merge", self.owner, self.repo, pr_number);
        let method = merge_method.unwrap_or("merge");
        let body = serde_json::json!({ "merge_method": method });

        let resp = self.client.request(reqwest::Method::PUT, &url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(GitHubError::ApiError(format!("{:?}", err)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Close a PR
    pub async fn close_pr(&self, pr_number: u64) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/pulls/{}", self.owner, self.repo, pr_number);
        let body = serde_json::json!({ "state": "closed" });

        let resp = self.client.patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    // ============ Issues ============

    /// List issues
    pub async fn list_issues(&self, state: Option<&str>, labels: Option<&str>) -> Result<Vec<Value>, GitHubError> {
        let mut url = format!("https://api.github.com/repos/{}/{}/issues", self.owner, self.repo);
        let mut params = Vec::new();
        if let Some(s) = state {
            params.push(format!("state={}", s));
        }
        if let Some(l) = labels {
            params.push(format!("labels={}", l));
        }
        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Get an issue
    pub async fn get_issue(&self, issue_number: u64) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/issues/{}", self.owner, self.repo, issue_number);
        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if resp.status() == 404 {
            return Err(GitHubError::NotFound(format!("Issue #{}", issue_number)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Create an issue
    pub async fn create_issue(&self, title: &str, body: Option<&str>, labels: Option<Vec<&str>>) -> Result<Value, GitHubError> {
        let mut issue_body = serde_json::json!({ "title": title });
        if let Some(b) = body {
            issue_body["body"] = serde_json::json!(b);
        }
        if let Some(l) = labels {
            issue_body["labels"] = serde_json::json!(l);
        }

        let resp = self.post_request("issues")
            .json(&issue_body)
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(GitHubError::ApiError(format!("{:?}", err)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Close an issue
    pub async fn close_issue(&self, issue_number: u64) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/issues/{}", self.owner, self.repo, issue_number);
        let body = serde_json::json!({ "state": "closed" });

        let resp = self.client.patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    // ============ Branches ============

    /// List branches
    pub async fn list_branches(&self) -> Result<Vec<Value>, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/branches", self.owner, self.repo);
        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Create a branch
    pub async fn create_branch(&self, name: &str, sha: &str) -> Result<Value, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/git/refs", self.owner, self.repo);
        let body = serde_json::json!({
            "ref": format!("refs/heads/{}", name),
            "sha": sha
        });

        let resp = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(GitHubError::ApiError(format!("{:?}", err)));
        }

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    /// Delete a branch
    pub async fn delete_branch(&self, name: &str) -> Result<(), GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/git/refs heads/{}", self.owner, self.repo, name);
        let resp = self.delete_request(&format!("git/refs/heads/{}", name))
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if !resp.status().is_success() {
            let err: Value = resp.json().await.unwrap_or_default();
            return Err(GitHubError::ApiError(format!("{:?}", err)));
        }

        Ok(())
    }

    // ============ Commits ============

    /// List commits
    pub async fn list_commits(&self, sha: Option<&str>, path: Option<&str>) -> Result<Vec<Value>, GitHubError> {
        let url = format!("https://api.github.com/repos/{}/{}/commits", self.owner, self.repo);
        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))
    }

    // ============ Files ============

    /// Get file contents
    pub async fn get_file(&self, path: &str, ref_: Option<&str>) -> Result<String, GitHubError> {
        let url = if let Some(r) = ref_ {
            format!("https://api.github.com/repos/{}/{}/contents/{}?ref={}", self.owner, self.repo, path, r)
        } else {
            format!("https://api.github.com/repos/{}/{}/contents/{}", self.owner, self.repo, path)
        };

        let resp = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "deva-github")
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if resp.status() == 404 {
            return Err(GitHubError::NotFound(path.into()));
        }

        let data: Value = resp.json().await.map_err(|e| GitHubError::ApiError(e.to_string()))?;

        if let Some(content) = data.get("content") {
            let encoded = content.as_str().unwrap_or("").replace("\n", "");
            let decoded_bytes = base64_decode(&encoded)?;
            let decoded = String::from_utf8(decoded_bytes)
                .map_err(|e| GitHubError::ApiError(e.to_string()))?;
            Ok(decoded)
        } else {
            Err(GitHubError::NotFound(path.into()))
        }
    }
}

fn base64_decode(input: &str) -> Result<Vec<u8>, GitHubError> {
    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|e| GitHubError::ApiError(e.to_string()))?;
    Ok(decoded)
}
