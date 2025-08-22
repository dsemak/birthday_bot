/// Handle search functionality
async fn handle_search(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    query: &str,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id.0;
    let filter = crate::database::models::BirthdayFilter {
        chat_id: Some(chat_id),
        name: Some(query.to_string()),
        limit: Some(5),
        ..Default::default()
    };

    match app_state
        .services
        .birthday_service()
        .get_birthdays(filter)
        .await
    {
        Ok(birthdays) => {
            let text = crate::utils::format_search_results(query, &birthdays);

            bot.send_message(msg.chat.id, text)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .reply_markup(super::commands::create_main_menu_keyboard())
                .await?;
        }
        Err(e) => {
            tracing::error!("Failed to search birthdays: {}", e);
            let text = crate::utils::format_error_message(&e.to_string());

            bot.send_message(msg.chat.id, text)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .reply_markup(super::commands::create_main_menu_keyboard())
                .await?;
        }
    }

    Ok(())
}
