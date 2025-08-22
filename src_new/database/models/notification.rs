use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Notification settings record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub chat_id: i64,
    pub notify_time: NaiveTime,
    pub notify_days_before: Vec<i32>,
    pub is_enabled: bool,
    pub message_template: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input data for creating notification settings
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateNotificationSettingsInput {
    pub chat_id: i64,

    pub notify_time: Option<String>,

    pub notify_days_before: Option<Vec<i32>>,

    pub is_enabled: Option<bool>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Message template must be between 1 and 500 characters"
    ))]
    pub message_template: Option<String>,
}

/// Input data for updating notification settings
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct UpdateNotificationSettingsInput {
    pub notify_time: Option<String>,

    pub notify_days_before: Option<Vec<i32>>,

    pub is_enabled: Option<bool>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Message template must be between 1 and 500 characters"
    ))]
    pub message_template: Option<String>,
}

/// Notification event for sending birthday reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub chat_id: i64,
    pub birthday_id: uuid::Uuid,
    pub birthday_name: String,
    pub birthday_username: Option<String>,
    pub message: String,
    pub scheduled_time: DateTime<Utc>,
    pub days_before: i32,
}

impl NotificationSettings {
    /// Get default notification settings for a chat
    pub fn default_for_chat(chat_id: i64) -> Self {
        Self {
            chat_id,
            notify_time: NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
            notify_days_before: vec![0], // Only on the birthday
            is_enabled: true,
            message_template: "🎉 Сегодня день рождения у {name}! {username}".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Format the notification time as HH:MM
    pub fn format_time(&self) -> String {
        self.notify_time.format("%H:%M").to_string()
    }

    /// Parse notification message template with placeholders
    pub fn format_message(&self, name: &str, username: Option<&str>) -> String {
        let username_display = username
            .filter(|u| !u.is_empty())
            .map(|u| {
                if u.starts_with('@') {
                    u.to_string()
                } else {
                    format!("@{u}")
                }
            })
            .unwrap_or_default();

        self.message_template
            .replace("{name}", name)
            .replace("{username}", &username_display)
    }

    /// Check if notifications should be sent for the given number of days before
    pub fn should_notify_for_days_before(&self, days_before: i32) -> bool {
        self.is_enabled && self.notify_days_before.contains(&days_before)
    }

    /// Get all notification days sorted
    pub fn get_sorted_notify_days(&self) -> Vec<i32> {
        let mut days = self.notify_days_before.clone();
        days.sort_unstable();
        days
    }
}

impl CreateNotificationSettingsInput {
    /// Convert to NotificationSettings with defaults
    pub fn to_notification_settings(&self) -> Result<NotificationSettings, String> {
        let notify_time = match &self.notify_time {
            Some(time_str) => parse_time_string(time_str)?,
            None => NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
        };

        let notify_days_before = self.notify_days_before.clone().unwrap_or_else(|| vec![0]);
        let is_enabled = self.is_enabled.unwrap_or(true);
        let message_template = self
            .message_template
            .clone()
            .unwrap_or_else(|| "🎉 Сегодня день рождения у {name}! {username}".to_string());

        Ok(NotificationSettings {
            chat_id: self.chat_id,
            notify_time,
            notify_days_before,
            is_enabled,
            message_template,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

/// Parse time string in HH:MM format
fn parse_time_string(time_str: &str) -> Result<NaiveTime, String> {
    if time_str.len() != 5
        || &time_str[2..3] != ":"
        || !time_str[..2].chars().all(|c| c.is_ascii_digit())
        || !time_str[3..].chars().all(|c| c.is_ascii_digit())
    {
        return Err(format!("Invalid time format: {time_str}. Expected HH:MM"));
    }

    NaiveTime::parse_from_str(time_str, "%H:%M")
        .map_err(|_| format!("Invalid time format: {time_str}. Expected HH:MM"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks if notification days are valid: non-empty, <=10, each in 0..=365.
    fn validate_notify_days(days: &Option<Vec<i32>>) -> Result<(), validator::ValidationError> {
        if let Some(days) = days {
            if days.is_empty() {
                return Err(validator::ValidationError::new("empty_notify_days"));
            }
            if days.len() > 10 {
                return Err(validator::ValidationError::new("too_many_notify_days"));
            }
            if days.iter().any(|&d| !(0..=365).contains(&d)) {
                return Err(validator::ValidationError::new("invalid_notify_day"));
            }
        }
        Ok(())
    }

    #[test]
    fn test_format_message() {
        let settings = NotificationSettings::default_for_chat(123);

        // Test with username
        let msg1 = settings.format_message("John Doe", Some("johndoe"));
        assert_eq!(msg1, "🎉 Сегодня день рождения у John Doe! @johndoe");

        // Test with username that already has @
        let msg2 = settings.format_message("Jane Smith", Some("@janesmith"));
        assert_eq!(msg2, "🎉 Сегодня день рождения у Jane Smith! @janesmith");

        // Test without username
        let msg3 = settings.format_message("Bob Johnson", None);
        assert_eq!(msg3, "🎉 Сегодня день рождения у Bob Johnson! ");

        // Test with empty username
        let msg4 = settings.format_message("Alice Brown", Some(""));
        assert_eq!(msg4, "🎉 Сегодня день рождения у Alice Brown! ");
    }

    #[test]
    fn test_time_parsing() {
        assert!(parse_time_string("07:00").is_ok());
        assert!(parse_time_string("23:59").is_ok());
        assert!(parse_time_string("00:00").is_ok());

        assert!(parse_time_string("25:00").is_err());
        assert!(parse_time_string("12:60").is_err());
        assert!(parse_time_string("7:00").is_err()); // Should be 07:00
        assert!(parse_time_string("invalid").is_err());
    }

    #[test]
    fn test_notify_days_validation() {
        assert!(validate_notify_days(&Some(vec![0])).is_ok());
        assert!(validate_notify_days(&Some(vec![0, 1, 7])).is_ok());
        assert!(validate_notify_days(&Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])).is_ok());

        assert!(validate_notify_days(&Some(vec![])).is_err()); // Empty
        assert!(validate_notify_days(&Some(vec![-1])).is_err()); // Negative
        assert!(validate_notify_days(&Some(vec![366])).is_err()); // Too large
        assert!(validate_notify_days(&Some(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])).is_err());
        // Too many
    }
}
