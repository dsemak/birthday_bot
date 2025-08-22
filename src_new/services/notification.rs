use chrono::{DateTime, NaiveTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

use crate::{
    database::{
        models::{Chat, CreateNotificationSettingsInput, NotificationEvent, NotificationSettings},
        repository::{BirthdayRepository, ChatRepository, NotificationRepository},
    },
    errors::BotResult,
};

/// Service for managing notifications
/// Follows Single Responsibility Principle - only handles notification logic
#[derive(Debug, Clone)]
pub struct NotificationService {
    notification_repo: NotificationRepository,
    chat_repo: ChatRepository,
    birthday_repo: BirthdayRepository,
}

impl NotificationService {
    /// Create new notification service
    pub fn new(
        notification_repo: NotificationRepository,
        chat_repo: ChatRepository,
        birthday_repo: BirthdayRepository,
    ) -> Self {
        Self {
            notification_repo,
            chat_repo,
            birthday_repo,
        }
    }

    /// Get notification settings for a chat
    pub async fn get_settings(&self, chat_id: i64) -> BotResult<NotificationSettings> {
        if let Some(settings) = self.notification_repo.get_by_chat_id(chat_id).await? {
            Ok(settings)
        } else {
            // Return default settings if none exist
            Ok(NotificationSettings::default_for_chat(chat_id))
        }
    }

    /// Update notification settings for a chat
    pub async fn update_settings(
        &self,
        input: CreateNotificationSettingsInput,
    ) -> BotResult<NotificationSettings> {
        self.notification_repo.upsert(input).await
    }

    /// Generate notification events for a specific date
    pub async fn generate_notification_events(
        &self,
        date: DateTime<Utc>,
    ) -> BotResult<Vec<NotificationEvent>> {
        use chrono::Datelike;

        let month = date.month() as u8;
        let day = date.day() as u8;

        // Get all birthdays for this date
        let birthdays = self
            .birthday_repo
            .get_birthdays_for_date(month, day)
            .await?;
        let mut events = Vec::new();

        for birthday in birthdays {
            // Get chat and notification settings
            if let Some(chat) = self.chat_repo.get_by_id(birthday.chat_id).await? {
                if !chat.is_active {
                    continue; // Skip inactive chats
                }

                let settings = self.get_settings(birthday.chat_id).await?;
                if !settings.is_enabled {
                    continue; // Skip if notifications disabled
                }

                // Check if we should notify for 0 days before (today)
                if settings.should_notify_for_days_before(0) {
                    let scheduled_time =
                        self.calculate_notification_time(&chat, &settings, date)?;
                    let message =
                        settings.format_message(&birthday.name, birthday.username.as_deref());

                    events.push(NotificationEvent {
                        chat_id: birthday.chat_id,
                        birthday_id: birthday.id,
                        birthday_name: birthday.name,
                        birthday_username: birthday.username,
                        message,
                        scheduled_time,
                        days_before: 0,
                    });
                }
            }
        }

        Ok(events)
    }

    /// Calculate when to send notification considering timezone
    fn calculate_notification_time(
        &self,
        chat: &Chat,
        settings: &NotificationSettings,
        date: DateTime<Utc>,
    ) -> BotResult<DateTime<Utc>> {
        // Parse timezone
        let tz: Tz = chat.timezone.parse().map_err(|_| {
            crate::errors::BotError::internal(format!("Invalid timezone: {}", chat.timezone))
        })?;

        // Convert date to chat timezone
        let local_date = date.with_timezone(&tz).date_naive();

        // Combine with notification time
        let local_datetime = local_date.and_time(settings.notify_time);

        // Convert back to UTC
        let utc_datetime = tz
            .from_local_datetime(&local_datetime)
            .single()
            .ok_or_else(|| crate::errors::BotError::internal("Ambiguous local time conversion"))?
            .with_timezone(&Utc);

        Ok(utc_datetime)
    }

    /// Check if it's time to send notifications
    pub async fn should_send_notifications_now(
        &self,
        current_time: DateTime<Utc>,
    ) -> BotResult<bool> {
        // Simple implementation: check if it's close to any notification time
        // In a real implementation, you'd use a job scheduler
        let minute = current_time.minute();
        Ok(minute == 0) // Send notifications at the top of each hour
    }

    /// Validate notification settings
    pub fn validate_settings(&self, input: &CreateNotificationSettingsInput) -> BotResult<()> {
        // Validate time format
        if let Some(time_str) = &input.notify_time {
            NaiveTime::parse_from_str(time_str, "%H:%M").map_err(|_| {
                crate::errors::BotError::validation(format!(
                    "Invalid time format: {}. Expected HH:MM",
                    time_str
                ))
            })?;
        }

        // Validate days before
        if let Some(days) = &input.notify_days_before {
            if days.is_empty() {
                return Err(crate::errors::BotError::validation(
                    "At least one notification day must be specified",
                ));
            }

            for &day in days {
                if day < 0 || day > 365 {
                    return Err(crate::errors::BotError::validation(format!(
                        "Invalid notification day: {}. Must be between 0 and 365",
                        day
                    )));
                }
            }
        }

        Ok(())
    }

    /// Enable notifications for a chat
    pub async fn enable_notifications(&self, chat_id: i64) -> BotResult<NotificationSettings> {
        let input = CreateNotificationSettingsInput {
            chat_id,
            notify_time: None,        // Use default
            notify_days_before: None, // Use default
            is_enabled: Some(true),
            message_template: None, // Use default
        };

        self.update_settings(input).await
    }

    /// Disable notifications for a chat
    pub async fn disable_notifications(&self, chat_id: i64) -> BotResult<NotificationSettings> {
        let input = CreateNotificationSettingsInput {
            chat_id,
            notify_time: None,
            notify_days_before: None,
            is_enabled: Some(false),
            message_template: None,
        };

        self.update_settings(input).await
    }
}

/// Factory for creating NotificationService instances
/// Follows Dependency Inversion Principle
pub struct NotificationServiceFactory;

impl NotificationServiceFactory {
    pub fn create(
        notification_repo: NotificationRepository,
        chat_repo: ChatRepository,
        birthday_repo: BirthdayRepository,
    ) -> NotificationService {
        NotificationService::new(notification_repo, chat_repo, birthday_repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timezone_conversion() {
        // Test notification time calculation with different timezones
    }

    #[tokio::test]
    async fn test_notification_generation() {
        // Test notification event generation
    }
}
