use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_message;
use crate::localization::Messages;
use crate::AppState;

/// Handle "List Birthdays" callback
///
/// Shows a list of birthdays for the current chat.
/// If editing the message fails, sends a plain text message as fallback.
pub async fn handle_list_birthdays(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id.0;
    let filter = crate::database::models::BirthdayFilter {
        chat_id: Some(chat_id),
        limit: Some(10),
        ..Default::default()
    };

    let result = app_state
        .services
        .birthday_service()
        .get_birthdays(filter)
        .await;

    let (text, _is_error) = match result {
        Ok(ref birthdays) => (crate::utils::format_birthday_list(birthdays), false),
        Err(ref e) => (crate::utils::format_error_message(&e.to_string()), true),
    };

    let edit_result = bot
        .edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(create_main_menu_keyboard())
        .await;

    if let Err(edit_error) = edit_result {
        // Fallback: send as a new message in plain text
        match result {
            Ok(ref birthdays) => {
                let fallback_text = if birthdays.is_empty() {
                    get_user_localized_message(&app_state, user_id, Messages::list_empty).await
                } else {
                    let list = birthdays
                        .iter()
                        .enumerate()
                        .map(|(i, b)| format!("{}. {} - {}", i + 1, b.name, b.format_date()))
                        .collect::<Vec<_>>()
                        .join("\n");

                    let title =
                        get_user_localized_message(&app_state, user_id, Messages::list_title).await;

                    format!("{title}\n\n{list}")
                };
                if let Err(e) = bot.send_message(msg.chat.id, fallback_text).await {
                    tracing::error!("Failed to send fallback message: {}", e);
                    return Err(e);
                }
            }
            Err(e) => {
                let simple_error = get_user_localized_message(&app_state, user_id, || {
                    Messages::simple_error(&e.to_string())
                })
                .await;

                if let Err(_e) = bot.send_message(msg.chat.id, simple_error).await {
                    return Err(edit_error);
                }
            }
        }
    }

    Ok(())
}
