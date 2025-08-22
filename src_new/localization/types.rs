use serde::{Deserialize, Serialize};

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Russian,
    English,
}

impl Language {
    /// Get language from Telegram language code
    pub fn from_telegram_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "ru" | "ru-ru" => Language::Russian,
            _ => Language::English,
        }
    }

    /// Get Telegram language code
    pub fn to_telegram_code(&self) -> &'static str {
        match self {
            Language::Russian => "ru",
            Language::English => "en",
        }
    }

    /// Get display name for the language
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Russian => "Русский",
            Language::English => "English",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// Localized text with fallback support
#[derive(Debug, Clone)]
pub struct LocalizedText {
    russian: String,
    english: String,
}

impl LocalizedText {
    /// Create new localized text
    pub fn new(russian: impl Into<String>, english: impl Into<String>) -> Self {
        Self {
            russian: russian.into(),
            english: english.into(),
        }
    }

    /// Get text for specific language
    pub fn get(&self, language: Language) -> &str {
        match language {
            Language::Russian => &self.russian,
            Language::English => &self.english,
        }
    }

    /// Get text with fallback to English if Russian is not available
    pub fn get_with_fallback(&self, language: Language) -> &str {
        match language {
            Language::Russian => &self.russian,
            Language::English => &self.english,
        }
    }
}

impl From<(String, String)> for LocalizedText {
    fn from((russian, english): (String, String)) -> Self {
        Self::new(russian, english)
    }
}

impl From<(&str, &str)> for LocalizedText {
    fn from((russian, english): (&str, &str)) -> Self {
        Self::new(russian, english)
    }
}
