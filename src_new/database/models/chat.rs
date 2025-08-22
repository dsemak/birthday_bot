use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use teloxide::types::{Chat as TelegramChat, ChatKind};

/// Chat type enum matching the database enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "chat_type", rename_all = "lowercase")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

/// Chat record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Chat {
    pub chat_id: i64,
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub is_active: bool,
    pub timezone: String,
    pub language_code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input data for creating a new chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatInput {
    pub chat_id: i64,
    pub chat_type: ChatType,
    pub title: Option<String>,
    pub username: Option<String>,
    pub timezone: Option<String>,
    pub language_code: Option<String>,
}

/// Input data for updating an existing chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatInput {
    pub title: Option<String>,
    pub username: Option<String>,
    pub is_active: Option<bool>,
    pub timezone: Option<String>,
    pub language_code: Option<String>,
}

impl ChatType {
    /// Convert from teloxide ChatKind
    pub fn from_telegram_chat(chat: &TelegramChat) -> Self {
        match &chat.kind {
            ChatKind::Private(_) => ChatType::Private,
            ChatKind::Public(_) => {
                // Determine type based on chat properties
                if chat.is_supergroup() {
                    ChatType::Supergroup
                } else if chat.is_channel() {
                    ChatType::Channel
                } else {
                    ChatType::Group
                }
            }
        }
    }

    /// Check if this chat type supports group features
    pub fn is_group_like(&self) -> bool {
        matches!(
            self,
            ChatType::Group | ChatType::Supergroup | ChatType::Channel
        )
    }

    /// Check if this chat type is private
    pub fn is_private(&self) -> bool {
        matches!(self, ChatType::Private)
    }
}

impl Chat {
    /// Get display name for the chat
    pub fn display_name(&self) -> String {
        self.title
            .clone()
            .or_else(|| self.username.clone())
            .unwrap_or_else(|| format!("Chat {}", self.chat_id))
    }

    /// Check if chat supports group features
    pub fn is_group_like(&self) -> bool {
        self.chat_type.is_group_like()
    }

    /// Check if chat is private
    pub fn is_private(&self) -> bool {
        self.chat_type.is_private()
    }

    /// Get timezone as chrono_tz::Tz
    pub fn get_timezone(&self) -> Result<chrono_tz::Tz, chrono_tz::ParseError> {
        self.timezone.parse()
    }
}

impl CreateChatInput {
    /// Create from teloxide Chat
    pub fn from_telegram_chat(chat: &TelegramChat) -> Self {
        let chat_type = ChatType::from_telegram_chat(chat);

        // Get title and username from chat
        let title = chat.title().map(|t| t.to_string());
        let username = chat.username().map(|u| u.to_string());

        Self {
            chat_id: chat.id.0,
            chat_type,
            title,
            username,
            timezone: None,      // Will be set to default
            language_code: None, // Will be set to default
        }
    }

    /// Set default values for optional fields
    pub fn with_defaults(mut self) -> Self {
        if self.timezone.is_none() {
            self.timezone = Some("UTC".to_string());
        }
        if self.language_code.is_none() {
            self.language_code = Some("ru".to_string());
        }
        self
    }
}

impl From<&TelegramChat> for CreateChatInput {
    fn from(chat: &TelegramChat) -> Self {
        Self::from_telegram_chat(chat).with_defaults()
    }
}
