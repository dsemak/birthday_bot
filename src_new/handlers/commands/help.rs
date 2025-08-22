use teloxide::prelude::*;

use crate::localization::Messages;
use crate::AppState;

/// Handle /help command
pub(crate) async fn handle_help(bot: Bot, msg: Message, app_state: AppState) -> ResponseResult<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);

    let help_text =
        crate::handlers::get_user_localized_message(&app_state, user_id, || Messages::help_title())
            .await;

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(super::create_main_menu_keyboard())
        .await?;

    Ok(())
}
