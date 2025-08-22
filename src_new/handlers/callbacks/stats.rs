use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_text;
use crate::localization::Messages;
use crate::AppState;

/// Handle stats callback from inline keyboard
pub async fn handle_stats_callback(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    _chat_id: i64,
) -> ResponseResult<()> {
    handle_stats_common(bot, msg, app_state, user_id, true).await
}

/// Handle /stats command
pub async fn handle_stats_command(
    bot: Bot,
    msg: Message,
    app_state: AppState,
) -> ResponseResult<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
    handle_stats_common(bot, msg, app_state, user_id, false).await
}

/// Common stats handling logic
async fn handle_stats_common(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    is_edit: bool,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id.0;

    match app_state
        .services
        .birthday_service()
        .get_chat_statistics(chat_id)
        .await
    {
        Ok(stats) => {
            // Create localized stats message
            let title = get_user_localized_text(&app_state, user_id, Messages::stats_title()).await;
            let text = format!(
                "{}\n\n🎂 {}: {}\n🎉 {}: {}\n📅 {}: {}",
                title,
                get_user_localized_text(&app_state, user_id, Messages::stats_total_birthdays())
                    .await,
                stats.total_birthdays,
                get_user_localized_text(&app_state, user_id, Messages::stats_birthdays_today())
                    .await,
                stats.birthdays_today,
                get_user_localized_text(&app_state, user_id, Messages::stats_with_year()).await,
                stats.birthdays_with_year
            );

            if is_edit {
                bot.edit_message_text(msg.chat.id, msg.id, text)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .reply_markup(create_main_menu_keyboard())
                    .await?;
            } else {
                bot.send_message(msg.chat.id, text)
                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                    .await?;
            }
        }
        Err(e) => {
            tracing::error!("Failed to get stats: {}", e);
            let text = get_user_localized_text(&app_state, user_id, Messages::stats_error()).await;

            if is_edit {
                bot.edit_message_text(msg.chat.id, msg.id, text)
                    .reply_markup(create_main_menu_keyboard())
                    .await?;
            } else {
                bot.send_message(msg.chat.id, text).await?;
            }
        }
    }

    Ok(())
}
