use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_text;
use crate::localization::Messages;
use crate::AppState;

/// Handle "Add Single" callback
///
/// Shows instructions for adding a single birthday and sets the conversation state.
pub async fn handle_add_single(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    if let Err(e) = app_state
        .services
        .conversation_service()
        .start_adding_single(user_id, chat_id)
        .await
    {
        tracing::error!("Failed to set conversation state: {}", e);
    }

    let text =
        get_user_localized_text(&app_state, user_id, Messages::add_single_format_title()).await;

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(create_main_menu_keyboard())
        .await?;

    Ok(())
}
