use teloxide::prelude::*;

use crate::localization::Messages;
use crate::AppState;

/// Handle /menu command
pub(crate) async fn handle_menu(bot: Bot, msg: Message, app_state: AppState) -> ResponseResult<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
    let menu_text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
        Messages::main_menu_title()
    })
    .await;

    bot.send_message(msg.chat.id, menu_text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(super::create_main_menu_keyboard())
        .await?;

    Ok(())
}
