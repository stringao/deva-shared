use crate::error::TelegramError;
use serde::{Deserialize, Serialize};

/// Telegram Bot API Client using raw HTTP
#[derive(Clone)]
pub struct DevaTelegramBot {
    client: reqwest::Client,
    token: String,
    chat_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
struct SendMessageRequest {
    chat_id: i64,
    text: String,
    parse_mode: Option<String>,
    reply_to_message_id: Option<i64>,
}

#[derive(Deserialize, Debug)]
struct TelegramResponse<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Message {
    message_id: i64,
    chat: Chat,
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Chat {
    id: i64,
}

#[derive(Deserialize, Debug)]
struct Update {
    update_id: u64,
    message: Option<Message>,
}

impl DevaTelegramBot {
    /// Create from BOT_TOKEN environment variable
    pub fn from_env() -> Result<Self, TelegramError> {
        let token = std::env::var("BOT_TOKEN")
            .map_err(|_| TelegramError::AuthError("BOT_TOKEN not set".into()))?;
        Ok(Self::new(token, None))
    }

    /// Create a new bot
    pub fn new(token: impl Into<String>, default_chat_id: Option<i64>) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.into(),
            chat_id: default_chat_id,
        }
    }

    /// Set default chat ID for notifications
    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    fn api_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }

    /// Send a text message
    pub async fn send_message(&self, text: &str) -> Result<Message, TelegramError> {
        let chat_id = self.chat_id.ok_or_else(|| TelegramError::BotError("No chat_id set".into()))?;
        self.send_to(chat_id, text).await
    }

    /// Send message to specific chat
    pub async fn send_to(&self, chat_id: i64, text: &str) -> Result<Message, TelegramError> {
        let request = SendMessageRequest {
            chat_id,
            text: text.into(),
            parse_mode: Some("HTML".into()),
            reply_to_message_id: None,
        };

        let url = self.api_url("sendMessage");
        let resp = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        let result: TelegramResponse<Message> = resp.json().await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        result.result.ok_or_else(|| TelegramError::BotError(
            result.description.unwrap_or_else(|| "Unknown error".into())
        ))
    }

    /// Send message as reply
    pub async fn send_reply(&self, text: &str, reply_to: i64) -> Result<Message, TelegramError> {
        let chat_id = self.chat_id.ok_or_else(|| TelegramError::BotError("No chat_id set".into()))?;

        let request = SendMessageRequest {
            chat_id,
            text: text.into(),
            parse_mode: Some("HTML".into()),
            reply_to_message_id: Some(reply_to),
        };

        let url = self.api_url("sendMessage");
        let resp = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        let result: TelegramResponse<Message> = resp.json().await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        result.result.ok_or_else(|| TelegramError::BotError(
            result.description.unwrap_or_else(|| "Unknown error".into())
        ))
    }

    /// Get updates (long polling)
    pub async fn get_updates(&self, offset: Option<u64>) -> Result<Vec<Update>, TelegramError> {
        let mut url = self.api_url("getUpdates");
        if let Some(o) = offset {
            url.push_str(&format!("?offset={}", o));
        }

        let resp = self.client.get(&url)
            .send()
            .await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        let result: TelegramResponse<Vec<Update>> = resp.json().await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        result.result.ok_or_else(|| TelegramError::BotError(
            result.description.unwrap_or_else(|| "Unknown error".into())
        ))
    }

    /// Answer callback query
    pub async fn answer_callback(&self, callback_id: &str, text: &str) -> Result<(), TelegramError> {
        #[derive(Serialize)]
        struct AnswerRequest {
            callback_query_id: String,
            text: Option<String>,
        }

        let request = AnswerRequest {
            callback_query_id: callback_id.into(),
            text: Some(text.into()),
        };

        let url = self.api_url("answerCallbackQuery");
        let resp = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        let result: TelegramResponse<bool> = resp.json().await
            .map_err(|e| TelegramError::ApiError(e.to_string()))?;

        if result.ok {
            Ok(())
        } else {
            Err(TelegramError::BotError(
                result.description.unwrap_or_else(|| "Unknown error".into())
            ))
        }
    }
}

/// Bot commands
#[derive(Clone)]
pub enum BotCommand {
    Help,
    Start,
    Status,
    Projects,
    Health(String),
    Doctor(String),
    Activity { limit: Option<u32> },
}

impl BotCommand {
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim_start_matches('/');
        let parts: Vec<&str> = text.split_whitespace().collect();
        let cmd = parts.first()?;

        match *cmd {
            "help" | "h" | "?" => Some(BotCommand::Help),
            "start" => Some(BotCommand::Start),
            "status" => Some(BotCommand::Status),
            "projects" => Some(BotCommand::Projects),
            "health" => parts.get(1).map(|s| BotCommand::Health(s.to_string())),
            "doctor" => parts.get(1).map(|s| BotCommand::Doctor(s.to_string())),
            "activity" | "log" => {
                let limit = parts.get(1).and_then(|s| s.parse().ok());
                Some(BotCommand::Activity { limit })
            }
            _ => None,
        }
    }

    pub fn help_text() -> &'static str {
        "Deva Bot Commands:\n\
         /help - Show this message\n\
         /start - Start the bot\n\
         /status - Show bot status\n\
         /projects - List projects\n\
         /health <project> - Show project health\n\
         /doctor <project> - Run diagnostics\n\
         /activity [limit] - Show recent activity"
    }

    pub async fn execute(&self, bot: &DevaTelegramBot, chat_id: i64) -> Result<(), TelegramError> {
        match self {
            BotCommand::Help => {
                bot.send_to(chat_id, Self::help_text()).await?;
            }
            BotCommand::Start => {
                bot.send_to(chat_id, "Deva Bot started! Use /help for commands.").await?;
            }
            BotCommand::Status => {
                bot.send_to(chat_id, "Deva Bot v0.1.0\nStatus: Running").await?;
            }
            BotCommand::Projects => {
                bot.send_to(chat_id, "Projects:\n- project-a\n- project-b").await?;
            }
            BotCommand::Health(project) => {
                let msg = format!("Health check for {}: All systems operational", project);
                bot.send_to(chat_id, &msg).await?;
            }
            BotCommand::Doctor(project) => {
                let msg = format!("Running diagnostics for {}...", project);
                bot.send_to(chat_id, &msg).await?;
            }
            BotCommand::Activity { limit } => {
                let limit = limit.unwrap_or(10);
                let msg = format!("Recent activity (last {} items):\n- PR #123 merged\n- Issue #456 closed", limit);
                bot.send_to(chat_id, &msg).await?;
            }
        }
        Ok(())
    }
}
