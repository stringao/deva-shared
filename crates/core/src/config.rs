use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevaConfig {
    pub github: GitHubConfig,
    pub azure_devops: AzureDevOpsConfig,
    pub telegram: TelegramConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureDevOpsConfig {
    pub organization: String,
    pub token: String,
    pub project: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub encryption_key: String,
}

impl DevaConfig {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }

    pub fn default_config() -> Self {
        Self {
            github: GitHubConfig {
                token: String::new(),
                owner: "stringao".to_string(),
                repo: "deva".to_string(),
            },
            azure_devops: AzureDevOpsConfig {
                organization: String::new(),
                token: String::new(),
                project: String::new(),
            },
            telegram: TelegramConfig {
                bot_token: String::new(),
                chat_id: String::new(),
            },
            database: DatabaseConfig {
                url: "sqlite:./deva.db".to_string(),
            },
            auth: AuthConfig {
                jwt_secret: String::new(),
                encryption_key: String::new(),
            },
        }
    }
}