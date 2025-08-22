use sqlx::{PgPool, Row};

use crate::{
    database::models::{Chat, CreateChatInput},
    errors::BotResult,
};

const UPSERT_CHAT_QUERY: &str = r#"
    INSERT INTO chats (chat_id, chat_type, title, username, timezone, language_code)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT (chat_id) DO UPDATE SET
        title = EXCLUDED.title,
        username = EXCLUDED.username,
        updated_at = NOW()
    RETURNING chat_id, chat_type, title, username, is_active, timezone, language_code, created_at, updated_at
"#;

const GET_CHAT_QUERY: &str = r#"
    SELECT chat_id, chat_type, title, username, is_active, timezone, language_code, created_at, updated_at FROM chats WHERE chat_id = $1
"#;

/// Simplified repository for managing chat data
#[derive(Debug, Clone)]
pub struct ChatRepository {
    pool: PgPool,
}

impl ChatRepository {
    /// Create new repository instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create or update chat
    pub async fn upsert(&self, input: CreateChatInput) -> BotResult<Chat> {
        let row = sqlx::query(UPSERT_CHAT_QUERY)
            .bind(input.chat_id)
            .bind(input.chat_type)
            .bind(&input.title)
            .bind(&input.username)
            .bind(input.timezone.unwrap_or_else(|| "UTC".to_string()))
            .bind(input.language_code.unwrap_or_else(|| "ru".to_string()))
            .fetch_one(&self.pool)
            .await?;

        let chat = Chat {
            chat_id: row.get("chat_id"),
            chat_type: row.get("chat_type"),
            title: row.get("title"),
            username: row.get("username"),
            is_active: row.get("is_active"),
            timezone: row.get("timezone"),
            language_code: row.get("language_code"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(chat)
    }

    /// Get chat by ID
    pub async fn get_by_id(&self, chat_id: i64) -> BotResult<Option<Chat>> {
        let row = sqlx::query(GET_CHAT_QUERY)
            .bind(chat_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let chat = Chat {
                chat_id: row.get("chat_id"),
                chat_type: row.get("chat_type"),
                title: row.get("title"),
                username: row.get("username"),
                is_active: row.get("is_active"),
                timezone: row.get("timezone"),
                language_code: row.get("language_code"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(chat))
        } else {
            Ok(None)
        }
    }
}
