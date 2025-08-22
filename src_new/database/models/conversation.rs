use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::BotResult;
use crate::localization;

/// Conversation state record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConversationState {
    pub user_id: i64,
    pub chat_id: i64,
    pub state: sqlx::types::Json<BotState>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Bot conversation states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BotState {
    MainMenu,
    AddingSingle {
        step: AddStep,
        data: PartialBirthday,
    },
    AddingBatch {
        step: BatchAddStep,
        data: Vec<PartialBirthday>,
    },
    EditingBirthday {
        birthday_id: Uuid,
        step: EditStep,
        original_data: PartialBirthday,
    },
    RemovingBirthday {
        step: RemoveStep,
        candidates: Vec<BirthdayCandidate>,
    },
    Settings {
        page: SettingsPage,
        data: PartialSettings,
    },
    Export {
        step: ExportStep,
        options: ExportOptions,
    },
}

/// Steps for adding a single birthday
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AddStep {
    Name,
    Date,
    Year,
    Username,
    Notes,
    Confirmation,
}

/// Steps for batch adding birthdays
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BatchAddStep {
    AwaitingFile,
    Confirmation { previews: Vec<BirthdayPreview> },
}

/// Steps for editing a birthday
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EditStep {
    SelectField,
    EnterValue { field: EditableField },
    Confirmation,
}

/// Steps for removing a birthday
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RemoveStep {
    SelectBirthday,
    Confirmation { selected_id: Uuid },
}

/// Settings pages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettingsPage {
    Main,
    Notifications,
    TimeZone,
    Language,
    MessageTemplate,
}

/// Export steps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportStep {
    SelectFormat,
    SelectOptions,
    Confirmation,
}

/// Partial birthday data during conversation
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PartialBirthday {
    pub name: Option<String>,
    pub birth_month: Option<u8>,
    pub birth_day: Option<u8>,
    pub birth_year: Option<u16>,
    pub username: Option<String>,
    pub notes: Option<String>,
}

/// Birthday preview for confirmation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BirthdayPreview {
    pub name: String,
    pub birth_month: u8,
    pub birth_day: u8,
    pub birth_year: Option<u16>,
    pub username: Option<String>,
    pub notes: Option<String>,
}

/// Birthday candidate for removal selection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BirthdayCandidate {
    pub id: Uuid,
    pub name: String,
    pub birth_date: String,
    pub username: Option<String>,
}

/// Editable fields for birthday editing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EditableField {
    Name,
    Date,
    Year,
    Username,
    Notes,
}

/// Partial settings data during conversation
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PartialSettings {
    pub notify_time: Option<String>,
    pub notify_days_before: Option<Vec<i32>>,
    pub is_enabled: Option<bool>,
    pub message_template: Option<String>,
    pub timezone: Option<String>,
    pub language_code: Option<String>,
}

/// Export options
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ExportOptions {
    pub format: Option<ExportFormat>,
    pub include_notes: bool,
    pub include_usernames: bool,
    pub sort_by: Option<ExportSortBy>,
}

/// Export formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
    Excel,
}

/// Export sorting options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportSortBy {
    Name,
    Date,
    CreatedAt,
}

impl BotState {
    /// Get the current conversation step as a string for display
    pub fn current_step(&self) -> &'static str {
        match self {
            BotState::MainMenu => "main_menu",
            BotState::AddingSingle { step, .. } => match step {
                AddStep::Name => "adding_name",
                AddStep::Date => "adding_date",
                AddStep::Year => "adding_year",
                AddStep::Username => "adding_username",
                AddStep::Notes => "adding_notes",
                AddStep::Confirmation => "adding_confirmation",
            },
            BotState::AddingBatch { step, .. } => match step {
                BatchAddStep::AwaitingFile => "batch_awaiting_file",
                BatchAddStep::Confirmation { .. } => "batch_confirmation",
            },
            BotState::EditingBirthday { step, .. } => match step {
                EditStep::SelectField => "editing_select_field",
                EditStep::EnterValue { .. } => "editing_enter_value",
                EditStep::Confirmation => "editing_confirmation",
            },
            BotState::RemovingBirthday { step, .. } => match step {
                RemoveStep::SelectBirthday => "removing_select",
                RemoveStep::Confirmation { .. } => "removing_confirmation",
            },
            BotState::Settings { page, .. } => match page {
                SettingsPage::Main => "settings_main",
                SettingsPage::Notifications => "settings_notifications",
                SettingsPage::TimeZone => "settings_timezone",
                SettingsPage::Language => "settings_language",
                SettingsPage::MessageTemplate => "settings_template",
            },
            BotState::Export { step, .. } => match step {
                ExportStep::SelectFormat => "export_format",
                ExportStep::SelectOptions => "export_options",
                ExportStep::Confirmation => "export_confirmation",
            },
        }
    }

    /// Check if the state requires user input
    pub fn requires_input(&self) -> bool {
        match self {
            BotState::MainMenu => false,
            BotState::AddingSingle { step, .. } => !matches!(step, AddStep::Confirmation),
            BotState::AddingBatch { step, .. } => matches!(step, BatchAddStep::AwaitingFile),
            BotState::EditingBirthday { step, .. } => {
                matches!(step, EditStep::EnterValue { .. })
            }
            BotState::RemovingBirthday { .. } => false,
            BotState::Settings { .. } => false,
            BotState::Export { .. } => false,
        }
    }
}

impl PartialBirthday {
    /// Check if all required fields are filled
    pub fn is_complete(&self) -> bool {
        self.name.is_some() && self.birth_month.is_some() && self.birth_day.is_some()
    }

    /// Validate the partial birthday fields and return a list of error messages.
    pub fn validate(&self, language: localization::Language) -> BotResult<()> {
        crate::utils::validation::validate_birthday(self, language)
    }

    /// Convert to BirthdayPreview
    pub fn to_preview(&self) -> BirthdayPreview {
        BirthdayPreview {
            name: self.name.clone().unwrap_or_default(),
            birth_month: self.birth_month.unwrap_or(1),
            birth_day: self.birth_day.unwrap_or(1),
            birth_year: self.birth_year,
            username: self.username.clone(),
            notes: self.notes.clone(),
        }
    }
}

impl ConversationState {
    /// Create new conversation state
    pub fn new(user_id: i64, chat_id: i64, state: BotState, ttl_minutes: i64) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            chat_id,
            state: sqlx::types::Json(state),
            expires_at: now + chrono::Duration::minutes(ttl_minutes),
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the conversation state is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Extend the expiration time
    pub fn extend_expiration(&mut self, additional_minutes: i64) {
        self.expires_at += chrono::Duration::minutes(additional_minutes);
        self.updated_at = Utc::now();
    }

    /// Update the state and expiration
    pub fn update_state(&mut self, new_state: BotState, ttl_minutes: i64) {
        self.state = sqlx::types::Json(new_state);
        self.expires_at = Utc::now() + chrono::Duration::minutes(ttl_minutes);
        self.updated_at = Utc::now();
    }
}
