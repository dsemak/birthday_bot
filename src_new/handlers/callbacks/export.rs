use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_text;
use crate::localization::Messages;
use crate::AppState;

/// Handle "Export Data" callback
///
/// Sends a localized message about export result or error.
/// If birthdays exist, prepares CSV data (not sent as file yet).
pub async fn handle_export_data(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    let filter = crate::database::models::BirthdayFilter {
        chat_id: Some(chat_id),
        limit: None,
        ..Default::default()
    };

    let birthdays = match app_state
        .services
        .birthday_service()
        .get_birthdays(filter)
        .await
    {
        Ok(birthdays) => birthdays,
        Err(e) => {
            tracing::error!("Failed to export data: {}", e);
            let text = get_user_localized_text(&app_state, user_id, Messages::export_error()).await;

            bot.edit_message_text(msg.chat.id, msg.id, text)
                .reply_markup(create_main_menu_keyboard())
                .await?;
            return Ok(());
        }
    };

    if birthdays.is_empty() {
        let text = get_user_localized_text(&app_state, user_id, Messages::export_no_data()).await;

        bot.edit_message_text(msg.chat.id, msg.id, text)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .reply_markup(create_main_menu_keyboard())
            .await?;
        return Ok(());
    }

    let csv_headers = get_user_localized_text(&app_state, user_id, Messages::csv_headers()).await;

    let mut csv_data = format!("{csv_headers}\n");
    for b in &birthdays {
        let year = b.birth_year.map(|y| y.to_string()).unwrap_or_default();
        let username = b.username.as_deref().unwrap_or("");
        let notes = b.notes.as_deref().unwrap_or("");
        csv_data.push_str(&format!(
            "{name},{day},{month},{year},{username},{notes}\n",
            name = b.name,
            day = b.birth_day,
            month = b.birth_month,
            year = year,
            username = username,
            notes = notes,
        ));
    }

    let text = get_user_localized_text(
        &app_state,
        user_id,
        Messages::export_success(birthdays.len() as i32),
    )
    .await;

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(create_main_menu_keyboard())
        .await?;

    Ok(())
}
