use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use teloxide::types::UserId;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub security: SecurityConfig,
    pub notifications: NotificationConfig,
    pub telegram: TelegramConfig,
    pub logging: LoggingConfig,
    pub metrics: MetricsConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub query_timeout: u64,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub ttl: u64,
    pub connection_timeout: u64,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub max_file_size: usize,
    pub max_birthdays_per_chat: usize,
    pub rate_limit_per_minute: u32,
    pub rate_limit_per_hour: u32,
    pub block_duration_minutes: u64,
    pub file_upload_cooldown_seconds: u64,
    pub admin_users: Vec<u64>,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub default_time: String,
    pub max_timezone_offset: i32,
    pub max_days_before: u32,
}

/// Telegram configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub token_env: String,
    pub webhook_url: String,
    pub webhook_port: u16,
    pub admin_users: Vec<u64>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub log_to_file: bool,
    pub log_directory: String,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub port: u16,
}

impl Settings {
    /// Load settings from configuration file and environment variables
    pub fn new() -> Result<Self, ConfigError> {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

        let config = Config::builder()
            // Start with the config file
            .add_source(File::with_name(&config_path).required(false))
            // Override with environment variables with prefix "BIRTHDAY_BOT"
            .add_source(Environment::with_prefix("BIRTHDAY_BOT").separator("__"))
            .build()?;

        let mut settings: Settings = config.try_deserialize()?;

        // Override token from environment if available
        if let Ok(_token) = std::env::var(&settings.telegram.token_env) {
            // Store token in memory, don't modify the config
            // Token will be retrieved separately
        }

        // Override database URL from environment if available
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            settings.database.url = db_url;
        }

        // Override Redis URL from environment if available
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            settings.redis.url = redis_url;
        }

        // Override admin users from environment if available
        if let Ok(admin_users_str) = std::env::var("ADMIN_USERS") {
            settings.security.admin_users = admin_users_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
        }

        // Override webhook URL from environment if available
        if let Ok(webhook_url) = std::env::var("WEBHOOK_URL") {
            settings.telegram.webhook_url = webhook_url;
        }

        Ok(settings)
    }

    /// Get Telegram bot token from environment
    pub fn get_telegram_token(&self) -> Result<String, ConfigError> {
        std::env::var(&self.telegram.token_env).map_err(|_| {
            ConfigError::Message(format!(
                "Environment variable '{}' not found",
                self.telegram.token_env
            ))
        })
    }

    /// Check if user is admin
    pub fn is_admin(&self, user_id: UserId) -> bool {
        self.security.admin_users.contains(&user_id.0)
            || self.telegram.admin_users.contains(&user_id.0)
    }

    /// Check if webhook mode is enabled
    pub fn is_webhook_mode(&self) -> bool {
        !self.telegram.webhook_url.is_empty()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate database URL
        if self.database.url.is_empty() {
            return Err(ConfigError::Message(
                "Database URL cannot be empty".to_string(),
            ));
        }

        // Validate Redis URL
        if self.redis.url.is_empty() {
            return Err(ConfigError::Message(
                "Redis URL cannot be empty".to_string(),
            ));
        }

        // Validate file size limit
        if self.security.max_file_size == 0 {
            return Err(ConfigError::Message(
                "Max file size must be greater than 0".to_string(),
            ));
        }

        // Validate rate limits
        if self.security.rate_limit_per_minute == 0 {
            return Err(ConfigError::Message(
                "Rate limit per minute must be greater than 0".to_string(),
            ));
        }

        // Validate notification time format
        if chrono::NaiveTime::parse_from_str(&self.notifications.default_time, "%H:%M").is_err() {
            return Err(ConfigError::Message(format!(
                "Invalid default notification time format: {}. Expected HH:MM",
                self.notifications.default_time
            )));
        }

        // Validate webhook port if webhook mode is enabled
        if self.is_webhook_mode() && self.telegram.webhook_port == 0 {
            return Err(ConfigError::Message("Invalid webhook port".to_string()));
        }

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                url: "postgresql://birthday_bot:password@localhost/birthday_bot".to_string(),
                max_connections: 10,
                connection_timeout: 30,
                query_timeout: 10,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                ttl: 3600,
                connection_timeout: 5,
            },
            security: SecurityConfig {
                max_file_size: 1024 * 1024, // 1MB
                max_birthdays_per_chat: 1000,
                rate_limit_per_minute: 10,
                rate_limit_per_hour: 100,
                block_duration_minutes: 15,
                file_upload_cooldown_seconds: 60,
                admin_users: vec![],
            },
            notifications: NotificationConfig {
                default_time: "07:00".to_string(),
                max_timezone_offset: 12,
                max_days_before: 30,
            },
            telegram: TelegramConfig {
                token_env: "TELEGRAM_BOT_TOKEN".to_string(),
                webhook_url: String::new(),
                webhook_port: 8080,
                admin_users: vec![],
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                log_to_file: true,
                log_directory: "logs".to_string(),
            },
            metrics: MetricsConfig {
                enabled: false,
                port: 9090,
            },
        }
    }
}
