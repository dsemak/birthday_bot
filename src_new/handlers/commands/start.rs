use teloxide::prelude::*;

use crate::AppState;

/// Handle /start command
pub(crate) async fn handle_start(
    bot: Bot,
    msg: Message,
    app_state: AppState,
) -> ResponseResult<()> {
    let user = msg.from().unwrap();
    let chat = &msg.chat;

    // Create or update user and chat using services
    let chat_input = crate::database::models::CreateChatInput::from(chat);

    if let Err(e) = app_state.services.user_service().upsert_user(user).await {
        tracing::error!("Failed to upsert user: {}", e);
        // Continue anyway, don't fail the command
    }

    if let Err(e) = app_state
        .services
        .chat_repository()
        .upsert(chat_input)
        .await
    {
        tracing::error!("Failed to upsert chat: {}", e);
        // Continue anyway, don't fail the command
    }

    let user_id = user.id.0 as i64;
    let welcome_template = crate::handlers::get_user_localized_text(
        &app_state,
        user_id,
        crate::localization::Messages::start_welcome(),
    )
    .await;
    let welcome_text = welcome_template.replace("{name}", &user.first_name);

    bot.send_message(msg.chat.id, welcome_text)
        .reply_markup(super::create_main_menu_keyboard())
        .await?;

    Ok(())
}
