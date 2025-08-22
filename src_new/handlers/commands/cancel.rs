use teloxide::prelude::*;

use crate::localization::Messages;
use crate::AppState;

/// Handle /cancel command
pub(crate) async fn handle_cancel(
    bot: Bot,
    msg: Message,
    app_state: AppState,
) -> ResponseResult<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or_default();
    let chat_id = msg.chat.id.0;

    // Clear current conversation state
    if let Err(e) = app_state
        .services
        .conversation_service()
        .clear_state(user_id, chat_id)
        .await
    {
        tracing::error!("Failed to clear conversation state: {}", e);
    }

    let cancel_text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
        Messages::operation_cancelled_success()
    })
    .await;

    bot.send_message(msg.chat.id, cancel_text)
        .reply_markup(super::create_main_menu_keyboard())
        .await?;

    Ok(())
}
