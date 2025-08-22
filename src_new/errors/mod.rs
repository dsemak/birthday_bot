use thiserror::Error;

/// User-facing errors that can be shown to users
#[derive(Error, Debug)]
pub enum UserError {
    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Authorization error: {message}")]
    Authorization { message: String },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("File too large: {size} bytes (max: {max_size} bytes)")]
    FileTooLarge { size: usize, max_size: usize },

    #[error("Invalid file format: {message}")]
    InvalidFileFormat { message: String },

    #[error("Resource limit exceeded: {message}")]
    ResourceLimit { message: String },

    #[error("Not found: {message}")]
    NotFound { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },

    #[error("Telegram error: {0}")]
    Telegram(#[from] teloxide::RequestError),
}

/// Legacy BotError for backward compatibility
#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Telegram error: {0}")]
    Telegram(#[from] teloxide::RequestError),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Authorization error: {message}")]
    Authorization { message: String },

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("File too large: {size} bytes (max: {max_size} bytes)")]
    FileTooLarge { size: usize, max_size: usize },

    #[error("Invalid file format: {message}")]
    InvalidFileFormat { message: String },

    #[error("Resource limit exceeded: {message}")]
    ResourceLimit { message: String },

    #[error("Not found: {message}")]
    NotFound { message: String },

    #[error("Internal error: {message}")]
    Internal { message: String },

    #[error("External service error: {service}: {message}")]
    ExternalService { service: String, message: String },
}

/// Result type alias for convenience
pub type BotResult<T> = Result<T, BotError>;

impl BotError {
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Create a resource limit error
    pub fn resource_limit(message: impl Into<String>) -> Self {
        Self::ResourceLimit {
            message: message.into(),
        }
    }

    /// Create an invalid file format error
    pub fn invalid_file_format(message: impl Into<String>) -> Self {
        Self::InvalidFileFormat {
            message: message.into(),
        }
    }
}

impl UserError {
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    /// Create a resource limit error
    pub fn resource_limit(message: impl Into<String>) -> Self {
        Self::ResourceLimit {
            message: message.into(),
        }
    }

    /// Create an invalid file format error
    pub fn invalid_file_format(message: impl Into<String>) -> Self {
        Self::InvalidFileFormat {
            message: message.into(),
        }
    }

    /// Create an internal error (should be used sparingly)
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
}

/// Convert BotError to UserError (only user-facing errors)
impl From<BotError> for UserError {
    fn from(error: BotError) -> Self {
        match error {
            BotError::Validation { message } => UserError::Validation { message },
            BotError::Authentication { message } => UserError::Authentication { message },
            BotError::Authorization { message } => UserError::Authorization { message },
            BotError::RateLimitExceeded => UserError::RateLimitExceeded,
            BotError::FileTooLarge { size, max_size } => UserError::FileTooLarge { size, max_size },
            BotError::InvalidFileFormat { message } => UserError::InvalidFileFormat { message },
            BotError::ResourceLimit { message } => UserError::ResourceLimit { message },
            BotError::NotFound { message } => UserError::NotFound { message },
            BotError::Telegram(e) => UserError::Telegram(e),
            // Internal errors should not be converted to user errors
            _ => UserError::Internal {
                message: "An internal error occurred".to_string(),
            },
        }
    }
}

/// Convert validation errors to BotError
impl From<validator::ValidationErrors> for BotError {
    fn from(err: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = err
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| {
                    if let Some(message) = &e.message {
                        format!("{}: {}", field, message)
                    } else {
                        format!("{}: invalid value", field)
                    }
                })
            })
            .collect();

        BotError::validation(messages.join(", "))
    }
}

/// Convert validation errors to UserError
impl From<validator::ValidationErrors> for UserError {
    fn from(err: validator::ValidationErrors) -> Self {
        let messages: Vec<String> = err
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| {
                    if let Some(message) = &e.message {
                        format!("{field}: {message}")
                    } else {
                        format!("{field}: invalid value")
                    }
                })
            })
            .collect();

        UserError::validation(messages.join(", "))
    }
}
