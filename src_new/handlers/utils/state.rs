use teloxide::RequestError;

use super::error::convert_bot_error;
use crate::database::models::BotState;
use crate::AppState;

/// Common logic for setting bot state
pub async fn set_bot_state(
    app_state: &AppState,
    user_id: i64,
    chat_id: i64,
    state: BotState,
    ttl_minutes: i64,
) -> Result<(), RequestError> {
    app_state
        .services
        .conversation_service()
        .set_state(user_id, chat_id, state, ttl_minutes)
        .await
        .map_err(convert_bot_error)
}

/// Common logic for clearing bot state
pub async fn clear_bot_state(
    app_state: &AppState,
    user_id: i64,
    chat_id: i64,
) -> Result<(), RequestError> {
    app_state
        .services
        .conversation_service()
        .clear_state(user_id, chat_id)
        .await
        .map_err(convert_bot_error)
}
