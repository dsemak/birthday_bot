pub mod callbacks;
pub mod commands;
pub mod messages;
pub mod utils;

use teloxide::{dispatching::UpdateHandler, prelude::*};

use crate::localization::{Language, LocalizedText};
use crate::AppState;

/// Create the main update handler
pub fn create_handler() -> UpdateHandler<teloxide::RequestError> {
    use teloxide::dptree;

    let command_handler = Update::filter_message().branch(
        dptree::entry()
            .filter_command::<commands::Command>()
            .endpoint(commands::command_handler),
    );

    let message_handler =
        Update::filter_message().branch(dptree::entry().endpoint(messages::message_handler));

    let callback_handler = Update::filter_callback_query()
        .branch(dptree::entry().endpoint(callbacks::callback_handler));

    dptree::entry()
        .branch(command_handler)
        .branch(message_handler)
        .branch(callback_handler)
}

/// Helper function to get user's localized message
pub async fn get_user_localized_message(
    app_state: &AppState,
    user_id: i64,
    message_fn: impl Fn() -> LocalizedText,
) -> String {
    match app_state
        .services
        .localization_service()
        .get_user_language(user_id)
        .await
    {
        Ok(language) => message_fn().get(language).to_string(),
        Err(_) => message_fn().get(Language::default()).to_string(),
    }
}

/// Helper function to get user's localized message from LocalizedText directly
pub async fn get_user_localized_text(
    app_state: &AppState,
    user_id: i64,
    text: LocalizedText,
) -> String {
    match app_state
        .services
        .localization_service()
        .get_user_language(user_id)
        .await
    {
        Ok(language) => text.get(language).to_string(),
        Err(_) => text.get(Language::default()).to_string(),
    }
}

/// Authentication filter for handlers
/// Rules:
/// - Private chats: any user can add birthdays
/// - Groups/Supergroups: only admins can add birthdays
/// - Channels: only admins can add birthdays
pub async fn auth_filter(msg: Message, app_state: AppState) -> bool {
    let user = match msg.from() {
        Some(user) => user,
        None => {
            tracing::warn!("No user in message");
            return false;
        }
    };

    let chat_id = msg.chat.id.0;
    let user_id = user.id.0 as i64;

    tracing::info!(
        "Auth check for user {} in chat {} (type: {:?})",
        user_id,
        chat_id,
        msg.chat
    );

    // Check if user is admin from config (always allow)
    if app_state.config.is_admin(user.id) {
        tracing::info!("User {} is admin from config", user_id);
        return true;
    }

    // Different rules based on chat type
    if msg.chat.is_private() {
        // Private chats: any user can add birthdays
        tracing::info!("Private chat - allowing user {}", user_id);
        true
    } else if msg.chat.is_group() || msg.chat.is_supergroup() {
        // Groups/Supergroups: only admins can add birthdays
        tracing::info!(
            "Group/Supergroup - checking admin status for user {}",
            user_id
        );
        let is_admin = check_if_user_is_admin(&app_state.bot, msg.chat.id, user.id).await;
        tracing::info!("User {} admin status in group: {}", user_id, is_admin);
        is_admin
    } else if msg.chat.is_channel() {
        // Channels: only admins can add birthdays
        tracing::info!("Channel - checking admin status for user {}", user_id);
        let is_admin = check_if_user_is_admin(&app_state.bot, msg.chat.id, user.id).await;
        tracing::info!("User {} admin status in channel: {}", user_id, is_admin);
        is_admin
    } else {
        // Unknown chat type - deny access
        tracing::warn!("Unknown chat type for chat {}", chat_id);
        false
    }
}

/// Check if user is admin in the chat
async fn check_if_user_is_admin(
    bot: &teloxide::Bot,
    chat_id: teloxide::types::ChatId,
    user_id: teloxide::types::UserId,
) -> bool {
    match bot.get_chat_member(chat_id, user_id).await {
        Ok(member) => {
            let is_admin = matches!(
                member.status(),
                teloxide::types::ChatMemberStatus::Owner
                    | teloxide::types::ChatMemberStatus::Administrator
            );
            tracing::debug!(
                "User {} admin status in chat {}: {}",
                user_id,
                chat_id,
                is_admin
            );
            is_admin
        }
        Err(e) => {
            tracing::error!(
                "Failed to check admin status for user {} in chat {}: {}",
                user_id,
                chat_id,
                e
            );
            false
        }
    }
}
