use sqlx::{PgPool, Row};

use crate::{database::models::ConversationState, errors::BotResult};

const GET_CONVERSATION_STATE_QUERY: &str = r#"
    SELECT user_id, chat_id, state, expires_at, created_at, updated_at FROM conversation_states WHERE user_id = $1 AND chat_id = $2 AND expires_at > NOW()
"#;

const SET_CONVERSATION_STATE_QUERY: &str = r#"
    INSERT INTO conversation_states (user_id, chat_id, state, expires_at)
    VALUES ($1, $2, $3, $4)
    ON CONFLICT (user_id, chat_id) DO UPDATE SET
        state = EXCLUDED.state,
        expires_at = EXCLUDED.expires_at,
        updated_at = NOW()
"#;

const CLEAR_CONVERSATION_STATE_QUERY: &str = r#"
    DELETE FROM conversation_states WHERE user_id = $1 AND chat_id = $2
"#;

/// Simplified repository for managing conversation states
#[derive(Debug, Clone)]
pub struct ConversationRepository {
    pool: PgPool,
}

impl ConversationRepository {
    /// Create new repository instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get conversation state
    pub async fn get_state(
        &self,
        user_id: i64,
        chat_id: i64,
    ) -> BotResult<Option<ConversationState>> {
        let row = sqlx::query(GET_CONVERSATION_STATE_QUERY)
            .bind(user_id)
            .bind(chat_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let state_json: serde_json::Value = row.get("state");
            let bot_state = serde_json::from_value(state_json).map_err(|e| {
                crate::errors::BotError::internal(format!(
                    "Failed to deserialize conversation state: {}",
                    e
                ))
            })?;

            let state = ConversationState {
                user_id: row.get("user_id"),
                chat_id: row.get("chat_id"),
                state: sqlx::types::Json(bot_state),
                expires_at: row.get("expires_at"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(state))
        } else {
            Ok(None)
        }
    }

    /// Set conversation state
    pub async fn set_state(&self, state: ConversationState) -> BotResult<()> {
        let state_json = serde_json::to_value(&state.state.0).map_err(|e| {
            crate::errors::BotError::internal(format!(
                "Failed to serialize conversation state: {}",
                e
            ))
        })?;

        sqlx::query(SET_CONVERSATION_STATE_QUERY)
            .bind(state.user_id)
            .bind(state.chat_id)
            .bind(state_json)
            .bind(state.expires_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Clear conversation state
    pub async fn clear_state(&self, user_id: i64, chat_id: i64) -> BotResult<bool> {
        let result = sqlx::query(CLEAR_CONVERSATION_STATE_QUERY)
            .bind(user_id)
            .bind(chat_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
