mod add_batch;
mod add_single;
mod default;

use teloxide::prelude::*;

use crate::database::models::BotState;
use crate::AppState;

use super::{auth_filter, get_user_localized_message, get_user_localized_text};

/// Send error message to user
async fn send_error_message(
    bot: &Bot,
    msg: &Message,
    app_state: &AppState,
    user_id: i64,
    message_fn: impl Fn() -> crate::localization::LocalizedText,
) -> ResponseResult<()> {
    let error_msg =
        crate::handlers::get_user_localized_message(app_state, user_id, message_fn).await;
    bot.send_message(msg.chat.id, error_msg).await?;
    Ok(())
}

/// Handle regular messages (non-command)
pub async fn message_handler(bot: Bot, msg: Message, app_state: AppState) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        tracing::debug!("Received message: {}", text);

        let user = match msg.from() {
            Some(user) => user,
            None => {
                tracing::error!("No user in message");
                return Ok(());
            }
        };

        let user_id = user.id.0 as i64;
        let chat_id = msg.chat.id.0;

        // Check authentication
        if !auth_filter(msg.clone(), app_state.clone()).await {
            let error_message = if msg.chat.is_private() {
                get_user_localized_message(&app_state, user_id, || {
                    crate::localization::Messages::no_permission_private()
                })
                .await
            } else {
                get_user_localized_message(&app_state, user_id, || {
                    crate::localization::Messages::no_permission_group()
                })
                .await
            };

            bot.send_message(msg.chat.id, error_message).await?;
            return Ok(());
        }

        // Get current conversation state
        let current_state = app_state
            .services
            .conversation_service()
            .get_state(user_id, chat_id)
            .await
            .unwrap_or(None);

        tracing::debug!(
            "Current state for user {} in chat {}: {:?}",
            user_id,
            chat_id,
            current_state
        );

        match current_state {
            Some(BotState::AddingSingle { step, data }) => {
                // Handle adding single birthday
                add_single::handle_adding_single(bot, msg.clone(), app_state, step, data, text)
                    .await?;
            }
            Some(BotState::AddingBatch { step, data }) => {
                // Handle batch adding
                add_batch::handle_adding_batch(bot, msg.clone(), app_state, step, data, text)
                    .await?;
            }
            Some(BotState::MainMenu) | None => {
                // Default behavior - try to parse as birthday or search
                default::handle_default_message(bot, msg.clone(), app_state, text).await?;
            }
            _ => {
                // Other states - send help
                let response = get_user_localized_text(
                    &app_state,
                    user_id,
                    crate::localization::Messages::welcome_message(),
                )
                .await;

                bot.send_message(msg.chat.id, response)
                    .reply_markup(super::commands::create_main_menu_keyboard())
                    .await?;
            }
        }
    }

    Ok(())
}
