use crate::{
    database::models::user::UpdateUserInput,
    localization::{Language, LocalizedText, Messages},
    services::UserService,
};
use anyhow::Result;
use std::sync::Arc;

/// Service for handling user localization preferences
#[derive(Debug, Clone)]
pub struct LocalizationService {
    user_service: Arc<UserService>,
}

impl LocalizationService {
    /// Create a new localization service
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    /// Get user's preferred language
    pub async fn get_user_language(&self, user_id: i64) -> Result<Language> {
        let user = self.user_service.get_user(user_id).await?;

        match user {
            Some(user) => match &user.language_code {
                Some(code) => Ok(Language::from_telegram_code(code)),
                None => Ok(Language::default()),
            },
            None => Ok(Language::default()),
        }
    }

    /// Set user's preferred language
    pub async fn set_user_language(&self, user_id: i64, language: Language) -> Result<()> {
        let update_input = UpdateUserInput {
            language_code: Some(language.to_telegram_code().to_string()),
            ..Default::default()
        };

        self.user_service.update_user(user_id, update_input).await?;

        Ok(())
    }

    /// Get localized text for user
    pub async fn get_text_for_user(
        &self,
        user_id: i64,
        text_fn: impl Fn() -> LocalizedText,
    ) -> Result<String> {
        let language = self.get_user_language(user_id).await?;
        Ok(text_fn().get(language).to_string())
    }

    /// Get localized message for user
    pub async fn get_message_for_user(
        &self,
        user_id: i64,
        message_fn: impl Fn() -> LocalizedText,
    ) -> Result<String> {
        self.get_text_for_user(user_id, message_fn).await
    }

    /// Get localized button text for user
    pub async fn get_button_text_for_user(
        &self,
        user_id: i64,
        button_fn: impl Fn() -> LocalizedText,
    ) -> Result<String> {
        self.get_text_for_user(user_id, button_fn).await
    }

    /// Get localized text with parameters for user
    pub async fn get_text_with_params_for_user<F, P>(
        &self,
        user_id: i64,
        text_fn: F,
        params: P,
    ) -> Result<String>
    where
        F: Fn(P) -> LocalizedText,
        P: Clone,
    {
        let language = self.get_user_language(user_id).await?;
        Ok(text_fn(params).get(language).to_string())
    }
}

impl Default for UpdateUserInput {
    fn default() -> Self {
        Self {
            username: None,
            first_name: None,
            last_name: None,
            language_code: None,
        }
    }
}
