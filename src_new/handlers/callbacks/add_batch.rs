use teloxide::requests::ResponseResult;
use teloxide::{prelude::*, types::Message};

use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_text;
use crate::localization::Messages;
use crate::AppState;

/// Handle "Add Batch" callback
///
/// Shows instructions for uploading a batch file and sets the conversation state.
pub async fn handle_adding_batch(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    let state = crate::database::models::BotState::AddingBatch {
        step: crate::database::models::BatchAddStep::AwaitingFile,
        data: Vec::new(),
    };
    if let Err(e) = app_state
        .services
        .conversation_service()
        .set_state(user_id, chat_id, state, 30)
        .await
    {
        tracing::error!("Failed to set conversation state: {}", e);
    }

    let text =
        get_user_localized_text(&app_state, user_id, Messages::add_batch_upload_file()).await;

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(create_main_menu_keyboard())
        .await?;

    Ok(())
}
