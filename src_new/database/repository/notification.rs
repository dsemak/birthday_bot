use sqlx::PgPool;

use crate::{
    database::models::{CreateNotificationSettingsInput, NotificationSettings},
    errors::BotResult,
};

const GET_NOTIFICATION_SETTINGS_QUERY: &str = r#"
    SELECT chat_id, notify_time, notify_days_before, is_enabled, message_template, created_at, updated_at FROM notification_settings WHERE chat_id = $1
"#;

const UPSERT_NOTIFICATION_SETTINGS_QUERY: &str = r#"
    INSERT INTO notification_settings (chat_id, notify_time, notify_days_before, is_enabled, message_template)
    VALUES ($1, $2, $3, $4, $5)
    ON CONFLICT (chat_id) DO UPDATE SET
        notify_time = EXCLUDED.notify_time,
        notify_days_before = EXCLUDED.notify_days_before,
        is_enabled = EXCLUDED.is_enabled,
        message_template = EXCLUDED.message_template,
        updated_at = NOW()
"#;

/// Repository for managing notification settings
#[derive(Debug, Clone)]
pub struct NotificationRepository {
    pool: PgPool,
}

impl NotificationRepository {
    /// Create new repository instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get notification settings for chat
    pub async fn get_by_chat_id(&self, chat_id: i64) -> BotResult<Option<NotificationSettings>> {
        let row = sqlx::query(GET_NOTIFICATION_SETTINGS_QUERY)
            .bind(chat_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            use sqlx::Row;
            let settings = NotificationSettings {
                chat_id: row.get("chat_id"),
                notify_time: row.get("notify_time"),
                notify_days_before: row.get("notify_days_before"),
                is_enabled: row.get("is_enabled"),
                message_template: row.get("message_template"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(settings))
        } else {
            Ok(None)
        }
    }

    /// Create or update notification settings
    pub async fn upsert(
        &self,
        input: CreateNotificationSettingsInput,
    ) -> BotResult<NotificationSettings> {
        validator::Validate::validate(&input)?;
        let settings = input
            .to_notification_settings()
            .map_err(crate::errors::BotError::validation)?;

        let row = sqlx::query(UPSERT_NOTIFICATION_SETTINGS_QUERY)
            .bind(settings.chat_id)
            .bind(settings.notify_time)
            .bind(&settings.notify_days_before)
            .bind(settings.is_enabled)
            .bind(&settings.message_template)
            .fetch_one(&self.pool)
            .await?;

        use sqlx::Row;
        let result = NotificationSettings {
            chat_id: row.get("chat_id"),
            notify_time: row.get("notify_time"),
            notify_days_before: row.get("notify_days_before"),
            is_enabled: row.get("is_enabled"),
            message_template: row.get("message_template"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(result)
    }
}
