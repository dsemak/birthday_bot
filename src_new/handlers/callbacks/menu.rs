use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_text;
use crate::localization::Messages;
use crate::AppState;

/// Handle "Main Menu" callback
pub async fn handle_main_menu(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    // Clear conversation state and return to main menu
    if let Err(e) = app_state
        .services
        .conversation_service()
        .clear_state(user_id, chat_id)
        .await
    {
        tracing::error!("Failed to clear conversation state: {}", e);
    }

    let text = get_user_localized_text(&app_state, user_id, Messages::main_menu_title()).await;

    // Get localized keyboard
    let keyboard = match create_localized_main_menu_keyboard(&app_state, user_id).await {
        Ok(keyboard) => keyboard,
        Err(e) => {
            tracing::error!("Failed to create localized keyboard: {}", e);
            create_main_menu_keyboard()
        }
    };

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

/// Create localized main menu keyboard
pub async fn create_localized_main_menu_keyboard(
    app_state: &AppState,
    user_id: i64,
) -> Result<teloxide::types::InlineKeyboardMarkup, anyhow::Error> {
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

    let add_single_text = app_state
        .services
        .localization_service()
        .get_button_text_for_user(user_id, Messages::add_single_button)
        .await?;

    let list_text = app_state
        .services
        .localization_service()
        .get_button_text_for_user(user_id, Messages::list_button)
        .await?;

    let search_text = app_state
        .services
        .localization_service()
        .get_button_text_for_user(user_id, Messages::search_button)
        .await?;

    let settings_text = app_state
        .services
        .localization_service()
        .get_button_text_for_user(user_id, Messages::settings_button)
        .await?;

    let export_text = app_state
        .services
        .localization_service()
        .get_button_text_for_user(user_id, Messages::export_button)
        .await?;

    let stats_text = app_state
        .services
        .localization_service()
        .get_button_text_for_user(user_id, Messages::stats_button)
        .await?;

    Ok(InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(add_single_text, "add_birthday"),
            InlineKeyboardButton::callback(list_text, "list_birthdays"),
        ],
        vec![
            InlineKeyboardButton::callback(search_text, "search_birthdays"),
            InlineKeyboardButton::callback(settings_text, "settings"),
        ],
        vec![
            InlineKeyboardButton::callback(export_text, "export_data"),
            InlineKeyboardButton::callback(stats_text, "stats"),
        ],
    ]))
}
