use crate::database::models::{BotState, ConversationState, PartialBirthday};
use crate::database::repository::ConversationRepository;
use crate::errors::BotError;

/// Service for managing conversation states
#[derive(Debug, Clone)]
pub struct ConversationService {
    repo: ConversationRepository,
}

impl ConversationService {
    /// Create a new conversation service
    pub fn new(repo: ConversationRepository) -> Self {
        Self { repo }
    }

    /// Get current conversation state for user in chat
    pub async fn get_state(
        &self,
        user_id: i64,
        chat_id: i64,
    ) -> Result<Option<BotState>, BotError> {
        let state = self.repo.get_state(user_id, chat_id).await?;
        Ok(state.map(|s| s.state.0))
    }

    /// Set conversation state for user in chat
    pub async fn set_state(
        &self,
        user_id: i64,
        chat_id: i64,
        state: BotState,
        ttl_minutes: i64,
    ) -> Result<(), BotError> {
        let conversation_state = ConversationState::new(user_id, chat_id, state, ttl_minutes);
        self.repo.set_state(conversation_state).await?;
        Ok(())
    }

    /// Clear conversation state for user in chat
    pub async fn clear_state(&self, user_id: i64, chat_id: i64) -> Result<(), BotError> {
        self.repo.clear_state(user_id, chat_id).await?;
        Ok(())
    }

    /// Start adding single birthday conversation
    pub async fn start_adding_single(&self, user_id: i64, chat_id: i64) -> Result<(), BotError> {
        let state = BotState::AddingSingle {
            step: crate::database::models::AddStep::Name,
            data: PartialBirthday::default(),
        };
        self.set_state(user_id, chat_id, state, 30).await
    }

    /// Start search conversation
    pub async fn start_search(&self, user_id: i64, chat_id: i64) -> Result<(), BotError> {
        let state = BotState::MainMenu; // For now, just return to main menu
        self.set_state(user_id, chat_id, state, 30).await
    }

    /// Check if user is in a specific state
    pub async fn is_in_state(
        &self,
        user_id: i64,
        chat_id: i64,
        expected_state: BotState,
    ) -> Result<bool, BotError> {
        let current_state = self.get_state(user_id, chat_id).await?;
        Ok(current_state.as_ref() == Some(&expected_state))
    }

    /// Check if user is in any input-requiring state
    pub async fn is_waiting_for_input(&self, user_id: i64, chat_id: i64) -> Result<bool, BotError> {
        let current_state = self.get_state(user_id, chat_id).await?;
        Ok(current_state.map(|s| s.requires_input()).unwrap_or(false))
    }
}
