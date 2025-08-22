use crate::{
    config::Settings,
    database::{
        models::{
            CreateChatMemberInput, CreateUserInput, UpdateUserInput, User, UserAction, UserRole, UserWithRole,
        },
        repository::UserRepository,
    },
    errors::BotResult,
};
use std::sync::Arc;
use teloxide::types::{User as TelegramUser, UserId};

/// Service for user management and authorization
/// Follows Single Responsibility Principle
#[derive(Debug, Clone)]
pub struct UserService {
    repository: UserRepository,
    config: Arc<Settings>,
}

impl UserService {
    /// Create new user service
    pub fn new(repository: UserRepository, config: Arc<Settings>) -> Self {
        Self { repository, config }
    }

    /// Register or update user from Telegram data
    pub async fn upsert_user(&self, telegram_user: &TelegramUser) -> BotResult<User> {
        let input = CreateUserInput::from(telegram_user);
        self.repository.upsert(input).await
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: i64) -> BotResult<Option<User>> {
        self.repository.get_by_id(user_id).await
    }

    /// Update user
    pub async fn update_user(&self, user_id: i64, input: UpdateUserInput) -> BotResult<User> {
        // First get current user
        let current_user = self.get_user(user_id).await?;
        let current_user = current_user.ok_or_else(|| {
            crate::errors::BotError::not_found("User not found")
        })?;

        // Create update input with current values where not provided
        let update_input = CreateUserInput {
            user_id,
            username: input.username.or(current_user.username),
            first_name: input.first_name.or(current_user.first_name),
            last_name: input.last_name.or(current_user.last_name),
            language_code: input.language_code.or(current_user.language_code),
            is_bot: current_user.is_bot,
        };

        self.repository.upsert(update_input).await
    }

    /// Add user to chat with appropriate role
    pub async fn add_user_to_chat(
        &self,
        chat_id: i64,
        user_id: i64,
        telegram_user: &TelegramUser,
    ) -> BotResult<UserWithRole> {
        // First ensure user exists
        self.upsert_user(telegram_user).await?;

        // Determine role based on configuration
        let role = self.determine_user_role(UserId(user_id as u64));

        let chat_member_input = CreateChatMemberInput {
            chat_id,
            user_id,
            role,
        };

        let _chat_member = self.repository.add_to_chat(chat_member_input).await?;

        // Get full user data with role
        self.repository
            .get_user_with_role(chat_id, user_id)
            .await?
            .ok_or_else(|| {
                crate::errors::BotError::internal("Failed to get user with role after creation")
            })
    }

    /// Check if user can perform an action
    pub async fn can_user_perform_action(
        &self,
        chat_id: i64,
        user_id: i64,
        action: UserAction,
    ) -> BotResult<bool> {
        // Check config-level admin
        if self.config.is_admin(UserId(user_id as u64)) {
            return Ok(true);
        }

        // Check database role
        if let Some(role) = self.repository.get_user_role(chat_id, user_id).await? {
            Ok(role.can_perform(&action))
        } else {
            // User not in chat - allow basic actions for private chats
            Ok(matches!(
                action,
                UserAction::ViewBirthdays | UserAction::AddBirthdays
            ))
        }
    }

    /// Get user with role information
    pub async fn get_user_with_role(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> BotResult<Option<UserWithRole>> {
        self.repository.get_user_with_role(chat_id, user_id).await
    }

    /// Determine user role based on configuration and context
    fn determine_user_role(&self, user_id: UserId) -> UserRole {
        if self.config.is_admin(user_id) {
            UserRole::Maintainer
        } else {
            UserRole::Member
        }
    }

    /// Check if user is authorized for admin actions
    pub async fn is_authorized_admin(&self, chat_id: i64, user_id: i64) -> BotResult<bool> {
        self.can_user_perform_action(chat_id, user_id, UserAction::ManageSettings)
            .await
    }

    /// Update user role (admin only)
    pub async fn update_user_role(
        &self,
        chat_id: i64,
        target_user_id: i64,
        new_role: UserRole,
        requesting_user_id: i64,
    ) -> BotResult<bool> {
        // Check authorization
        if !self
            .is_authorized_admin(chat_id, requesting_user_id)
            .await?
        {
            return Err(crate::errors::BotError::authorization(
                "Insufficient permissions to change user roles",
            ));
        }

        // Prevent non-maintainers from creating maintainers
        if new_role == UserRole::Maintainer
            && !self.config.is_admin(UserId(requesting_user_id as u64))
        {
            return Err(crate::errors::BotError::authorization(
                "Only maintainers can assign maintainer role",
            ));
        }

        self.repository
            .update_role(chat_id, target_user_id, new_role)
            .await
    }
}

/// Factory for creating UserService instances
/// Follows Dependency Inversion Principle
pub struct UserServiceFactory;

impl UserServiceFactory {
    pub fn create(repository: UserRepository, config: Arc<Settings>) -> UserService {
        UserService::new(repository, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests for authorization logic
    #[tokio::test]
    async fn test_admin_role_determination() {
        // Test admin role assignment based on config
    }

    #[tokio::test]
    async fn test_permission_checking() {
        // Test various permission scenarios
    }
}
