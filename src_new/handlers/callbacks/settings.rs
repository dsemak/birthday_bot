use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::get_user_localized_text;
use crate::localization::Messages;
use crate::AppState;

/// Handle "Settings" callback
pub async fn handle_settings(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    _chat_id: i64,
) -> ResponseResult<()> {
    // Get localized text for user
    let text = get_user_localized_text(&app_state, user_id, Messages::settings_title()).await;

    // Get localized keyboard
    let keyboard = create_localized_settings_keyboard(&app_state, user_id).await;

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

/// Create localized settings keyboard
async fn create_localized_settings_keyboard(
    app_state: &AppState,
    user_id: i64,
) -> teloxide::types::InlineKeyboardMarkup {
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

    let language_text =
        get_user_localized_text(app_state, user_id, Messages::language_setting()).await;

    let back_text = get_user_localized_text(app_state, user_id, Messages::back_button()).await;

    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            language_text,
            "settings_language",
        )],
        vec![InlineKeyboardButton::callback(back_text, "main_menu")],
    ])
}
