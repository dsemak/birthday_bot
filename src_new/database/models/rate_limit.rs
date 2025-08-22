use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

/// Rate limit record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RateLimit {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub action_type: String,
    pub window_start: DateTime<Utc>,
    pub request_count: i32,
    pub is_blocked: bool,
    pub blocked_until: Option<DateTime<Utc>>,
}

/// Rate limit configuration for different action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_requests_per_minute: u32,
    pub max_requests_per_hour: u32,
    pub block_duration_minutes: u64,
}

/// Rate limit action types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimitAction {
    AddBirthday,
    RemoveBirthday,
    EditBirthday,
    FileUpload,
    Command,
    Export,
}

/// Rate limit check result
#[derive(Debug, Clone)]
pub enum RateLimitResult {
    Allowed,
    Limited {
        retry_after: DateTime<Utc>,
        current_count: u32,
        max_count: u32,
    },
    Blocked {
        blocked_until: DateTime<Utc>,
        reason: String,
    },
}

/// Rate limit context for checking limits
#[derive(Debug, Clone)]
pub struct RateLimitContext {
    pub user_id: i64,
    pub chat_id: Option<i64>,
    pub action: RateLimitAction,
    pub ip_address: Option<String>,
}

impl RateLimitAction {
    /// Get string representation for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            RateLimitAction::AddBirthday => "add_birthday",
            RateLimitAction::RemoveBirthday => "remove_birthday",
            RateLimitAction::EditBirthday => "edit_birthday",
            RateLimitAction::FileUpload => "file_upload",
            RateLimitAction::Command => "command",
            RateLimitAction::Export => "export",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "add_birthday" => Some(RateLimitAction::AddBirthday),
            "remove_birthday" => Some(RateLimitAction::RemoveBirthday),
            "edit_birthday" => Some(RateLimitAction::EditBirthday),
            "file_upload" => Some(RateLimitAction::FileUpload),
            "command" => Some(RateLimitAction::Command),
            "export" => Some(RateLimitAction::Export),
            _ => None,
        }
    }

    /// Get default rate limit configuration for this action
    pub fn default_config(&self) -> RateLimitConfig {
        match self {
            RateLimitAction::AddBirthday => RateLimitConfig {
                max_requests_per_minute: 5,
                max_requests_per_hour: 50,
                block_duration_minutes: 15,
            },
            RateLimitAction::RemoveBirthday => RateLimitConfig {
                max_requests_per_minute: 5,
                max_requests_per_hour: 30,
                block_duration_minutes: 15,
            },
            RateLimitAction::EditBirthday => RateLimitConfig {
                max_requests_per_minute: 10,
                max_requests_per_hour: 100,
                block_duration_minutes: 10,
            },
            RateLimitAction::FileUpload => RateLimitConfig {
                max_requests_per_minute: 2,
                max_requests_per_hour: 10,
                block_duration_minutes: 30,
            },
            RateLimitAction::Command => RateLimitConfig {
                max_requests_per_minute: 20,
                max_requests_per_hour: 500,
                block_duration_minutes: 5,
            },
            RateLimitAction::Export => RateLimitConfig {
                max_requests_per_minute: 1,
                max_requests_per_hour: 5,
                block_duration_minutes: 60,
            },
        }
    }
}

impl RateLimit {
    /// Check if the rate limit is currently active (within the time window)
    pub fn is_active(&self, window_duration_minutes: u64) -> bool {
        let window_duration = chrono::Duration::minutes(window_duration_minutes as i64);
        Utc::now() - self.window_start < window_duration
    }

    /// Check if the user is currently blocked
    pub fn is_blocked(&self) -> bool {
        if !self.is_blocked {
            return false;
        }

        if let Some(blocked_until) = self.blocked_until {
            Utc::now() < blocked_until
        } else {
            true // Permanently blocked
        }
    }

    /// Get time until unblocked
    pub fn time_until_unblocked(&self) -> Option<chrono::Duration> {
        if !self.is_blocked() {
            return None;
        }

        if let Some(blocked_until) = self.blocked_until {
            let remaining = blocked_until - Utc::now();
            if remaining > chrono::Duration::zero() {
                Some(remaining)
            } else {
                None
            }
        } else {
            None // Permanently blocked
        }
    }
}

impl RateLimitResult {
    /// Check if the request is allowed
    pub fn is_allowed(&self) -> bool {
        matches!(self, RateLimitResult::Allowed)
    }

    /// Get retry after time if rate limited
    pub fn retry_after(&self) -> Option<DateTime<Utc>> {
        match self {
            RateLimitResult::Limited { retry_after, .. } => Some(*retry_after),
            RateLimitResult::Blocked { blocked_until, .. } => Some(*blocked_until),
            RateLimitResult::Allowed => None,
        }
    }

    /// Get user-friendly error message in Russian
    pub fn error_message(&self) -> Option<String> {
        match self {
            RateLimitResult::Allowed => None,
            RateLimitResult::Limited { retry_after, .. } => {
                let duration = *retry_after - Utc::now();
                let minutes = duration.num_minutes();
                let seconds = duration.num_seconds() % 60;

                if minutes > 0 {
                    Some(format!(
                        "Слишком много запросов. Попробуйте снова через {minutes} мин {seconds} сек",
                    ))
                } else {
                    Some(format!(
                        "Слишком много запросов. Попробуйте снова через {seconds} сек",
                    ))
                }
            }
            RateLimitResult::Blocked {
                blocked_until,
                reason,
            } => {
                let duration = *blocked_until - Utc::now();
                let hours = duration.num_hours();
                let minutes = duration.num_minutes() % 60;

                if hours > 0 {
                    Some(format!(
                        "Вы заблокированы на {hours} ч {minutes} мин. Причина: {reason}",
                    ))
                } else {
                    Some(format!(
                        "Вы заблокированы на {minutes} мин. Причина: {reason}",
                    ))
                }
            }
        }
    }
}

impl RateLimitContext {
    /// Create new context for user action
    pub fn new(user_id: i64, chat_id: Option<i64>, action: RateLimitAction) -> Self {
        Self {
            user_id,
            chat_id,
            action,
            ip_address: None,
        }
    }

    /// Create context with IP address
    pub fn with_ip(mut self, ip_address: Option<String>) -> Self {
        self.ip_address = ip_address;
        self
    }

    /// Get unique key for this rate limit context
    pub fn get_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.user_id,
            self.chat_id.unwrap_or(0),
            self.action.as_str()
        )
    }
}

/// In-memory rate limit cache for fast lookups
#[derive(Debug, Default)]
pub struct RateLimitCache {
    cache: HashMap<String, RateLimit>,
}

impl RateLimitCache {
    /// Create new cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Get rate limit from cache
    pub fn get(&self, key: &str) -> Option<&RateLimit> {
        self.cache.get(key)
    }

    /// Set rate limit in cache
    pub fn set(&mut self, key: String, rate_limit: RateLimit) {
        self.cache.insert(key, rate_limit);
    }

    /// Remove expired entries from cache
    pub fn cleanup_expired(&mut self) {
        let now = Utc::now();
        self.cache.retain(|_, rate_limit| {
            // Keep if it's blocked or if it's within the last hour
            rate_limit.is_blocked() || (now - rate_limit.window_start).num_hours() < 1
        });
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
