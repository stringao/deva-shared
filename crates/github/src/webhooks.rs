use crate::error::GitHubError;
use crate::models::*;
use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Webhook event handler
#[derive(Clone)]
pub struct WebhookHandler {
    pub secret: String,
    handlers: Arc<RwLock<Vec<WebhookEventHandler>>>,
}

#[derive(Clone)]
pub struct WebhookEventHandler {
    pub event: String,
    pub callback: Arc<dyn Fn(WebhookPayload) -> Result<(), GitHubError> + Send + Sync>,
}

impl WebhookHandler {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a handler for a specific event type
    pub async fn on(&self, event: impl Into<String>, handler: impl Fn(WebhookPayload) -> Result<(), GitHubError> + Send + Sync + 'static) {
        let mut handlers = self.handlers.write().await;
        handlers.push(WebhookEventHandler {
            event: event.into(),
            callback: Arc::new(handler),
        });
    }

    /// Verify and parse a webhook payload (simplified - in production use HMAC verification)
    pub fn verify(&self, payload: &[u8], _signature: &str) -> Result<WebhookPayload, GitHubError> {
        // In production, verify HMAC-SHA256 signature:
        // let computed = hmac_sha256::compute(&self.secret, payload);
        // if format!("sha256={}", hex::encode(computed)) != signature { ... }

        serde_json::from_slice(payload)
            .map_err(|e| GitHubError::InvalidPayload(e.to_string()))
    }

    /// Process a webhook payload
    pub async fn process(&self, payload: WebhookPayload) -> Result<(), GitHubError> {
        let event_type = payload.action.clone().unwrap_or_default();

        let handlers = self.handlers.read().await;
        for handler in handlers.iter() {
            if handler.event == event_type || handler.event == "*" {
                if let Err(e) = (handler.callback)(payload.clone()) {
                    tracing::error!("Webhook handler error: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Create an Axum router for webhooks
    pub fn router(handler: Arc<WebhookHandler>) -> Router {
        Router::new()
            .route("/webhook", post(handle_webhook))
            .with_state(handler)
    }
}

#[derive(Deserialize)]
struct WebhookHeaders {
    #[serde(rename = "X-Hub-Signature-256")]
    signature: Option<String>,
    #[serde(rename = "X-GitHub-Event")]
    event: Option<String>,
}

async fn handle_webhook(
    State(handler): State<Arc<WebhookHandler>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<WebhookPayload>,
) -> Result<String, axum::http::StatusCode> {
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let event = headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    info!("Received GitHub webhook: {} - {:?}", event, payload.action);

    // Note: In production, verify HMAC signature before processing
    if let Err(e) = handler.process(payload).await {
        tracing::error!("Webhook processing error: {}", e);
    }

    Ok("OK".to_string())
}
