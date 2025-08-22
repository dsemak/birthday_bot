use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use teloxide::types::User as TelegramUser;

/// User role enum matching the database enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Maintainer,
    Admin,
    Member,
    Restricted,
}

/// User record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub language_code: Option<String>,
    pub is_bot: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Chat member record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ChatMember {
    pub chat_id: i64,
    pub user_id: i64,
    pub role: UserRole,
    pub joined_at: DateTime<Utc>,
}

/// Combined user and role information  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRole {
    pub user: User,
    pub role: UserRole,
    pub joined_at: DateTime<Utc>,
}

/// Input data for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserInput {
    pub user_id: i64,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub language_code: Option<String>,
    pub is_bot: bool,
}

/// Input data for updating an existing user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub language_code: Option<String>,
}

/// Input data for creating a chat member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatMemberInput {
    pub chat_id: i64,
    pub user_id: i64,
    pub role: UserRole,
}

/// Input data for updating a chat member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatMemberInput {
    pub role: UserRole,
}

impl UserRole {
    /// Check if this role has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Maintainer)
    }

    /// Check if this role is maintainer
    pub fn is_maintainer(&self) -> bool {
        matches!(self, UserRole::Maintainer)
    }

    /// Check if this role can perform the given action
    pub fn can_perform(&self, action: &UserAction) -> bool {
        match action {
            UserAction::ViewBirthdays => !matches!(self, UserRole::Restricted),
            UserAction::AddBirthdays => !matches!(self, UserRole::Restricted),
            UserAction::EditBirthdays => self.is_admin(),
            UserAction::DeleteBirthdays => self.is_admin(),
            UserAction::ManageSettings => self.is_admin(),
            UserAction::ExportData => self.is_admin(),
            UserAction::ViewAuditLog => self.is_maintainer(),
            UserAction::ManageUsers => self.is_maintainer(),
            UserAction::ViewStats => true, // All users can view stats
        }
    }
}

/// Actions that users can perform
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserAction {
    ViewBirthdays,
    AddBirthdays,
    EditBirthdays,
    DeleteBirthdays,
    ManageSettings,
    ExportData,
    ViewAuditLog,
    ManageUsers,
    ViewStats,
}

impl User {
    /// Get display name for the user
    pub fn display_name(&self) -> String {
        let mut name_parts = Vec::new();

        if let Some(first_name) = &self.first_name {
            if !first_name.is_empty() {
                name_parts.push(first_name.clone());
            }
        }

        if let Some(last_name) = &self.last_name {
            if !last_name.is_empty() {
                name_parts.push(last_name.clone());
            }
        }

        if name_parts.is_empty() {
            if let Some(username) = &self.username {
                format!("@{username}")
            } else {
                let user_id = self.user_id;
                format!("User {user_id}")
            }
        } else {
            name_parts.join(" ")
        }
    }

    /// Get username with @ prefix if available
    pub fn username_display(&self) -> Option<String> {
        self.username.as_ref().map(|username| {
            if username.starts_with('@') {
                username.clone()
            } else {
                format!("@{username}")
            }
        })
    }

    /// Get full display name with username
    pub fn full_display_name(&self) -> String {
        let display_name = self.display_name();
        match self.username_display() {
            Some(username) => format!("{display_name} ({username})"),
            None => display_name,
        }
    }
}

impl CreateUserInput {
    /// Create from teloxide User
    pub fn from_telegram_user(user: &TelegramUser) -> Self {
        Self {
            user_id: user.id.0 as i64,
            username: user.username.clone(),
            first_name: Some(user.first_name.clone()),
            last_name: user.last_name.clone(),
            language_code: user.language_code.clone(),
            is_bot: user.is_bot,
        }
    }
}

impl From<&TelegramUser> for CreateUserInput {
    fn from(user: &TelegramUser) -> Self {
        Self::from_telegram_user(user)
    }
}

impl ChatMember {
    /// Check if this member has admin privileges
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Check if this member is maintainer
    pub fn is_maintainer(&self) -> bool {
        self.role.is_maintainer()
    }

    /// Check if this member can perform the given action
    pub fn can_perform(&self, action: &UserAction) -> bool {
        self.role.can_perform(action)
    }
}

impl UserWithRole {
    /// Check if this user has admin privileges
    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }

    /// Check if this user is maintainer
    pub fn is_maintainer(&self) -> bool {
        self.role.is_maintainer()
    }

    /// Check if this user can perform the given action
    pub fn can_perform(&self, action: &UserAction) -> bool {
        self.role.can_perform(action)
    }
}
