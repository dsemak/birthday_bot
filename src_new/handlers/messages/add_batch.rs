use teloxide::prelude::*;
use teloxide::types::Document;

use crate::database::models::{BatchAddStep, BotState, PartialBirthday};
use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::utils::*;
use crate::localization::Messages;
use crate::utils;
use crate::AppState;

use super::send_error_message;

/// Handle batch adding birthdays from file
pub async fn handle_adding_batch(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    step: BatchAddStep,
    _data: Vec<PartialBirthday>,
    text: &str,
) -> ResponseResult<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or_default();
    let chat_id = msg.chat.id.0;

    match step {
        BatchAddStep::AwaitingFile => {
            if let Some(document) = msg.document() {
                handle_document_upload(
                    bot,
                    msg.clone(),
                    app_state,
                    document.clone(),
                    user_id,
                    chat_id,
                )
                .await
            } else {
                handle_text_upload(bot, msg, app_state, text, user_id, chat_id).await
            }
        }
        BatchAddStep::Confirmation { previews } => {
            handle_confirmation_step(bot, msg, app_state, text, user_id, chat_id, previews).await
        }
    }
}

/// Handle document upload for batch processing
async fn handle_document_upload(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    document: Document,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    let file_name = document.file_name.as_deref().unwrap_or("");

    if !is_valid_file_format(file_name) {
        return send_error_message(&bot, &msg, &app_state, user_id, || {
            Messages::invalid_file_format_error()
        })
        .await;
    }

    // TODO: Implement proper file download
    let file_content = Vec::new();

    let birthdays = parse_file_content(&file_content, file_name)?;

    if birthdays.is_empty() {
        return send_error_message(&bot, &msg, &app_state, user_id, || {
            Messages::no_birthdays_found()
        })
        .await;
    }

    show_batch_confirmation(bot, msg, app_state, birthdays, user_id, chat_id).await
}

/// Handle text upload for batch processing
async fn handle_text_upload(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    text: &str,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    let birthdays = parse_text_content(text)?;

    if birthdays.is_empty() {
        return send_error_message(&bot, &msg, &app_state, user_id, || {
            Messages::format_insufficient_data()
        })
        .await;
    }

    show_batch_confirmation(bot, msg, app_state, birthdays, user_id, chat_id).await
}

/// Handle confirmation for batch adding
async fn handle_confirmation_step(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    text: &str,
    user_id: i64,
    chat_id: i64,
    _previews: Vec<crate::database::models::BirthdayPreview>,
) -> ResponseResult<()> {
    handle_confirmation_response(
        text,
        confirm_batch_save(
            bot.clone(),
            msg.clone(),
            app_state.clone(),
            user_id,
            chat_id,
        ),
        cancel_batch_operation(
            bot.clone(),
            msg.clone(),
            app_state.clone(),
            user_id,
            chat_id,
        ),
        send_invalid_input_error(bot, msg, app_state, user_id),
    )
    .await
}

// Helper functions

/// Check if file format is valid
fn is_valid_file_format(file_name: &str) -> bool {
    file_name.ends_with(".json") || file_name.ends_with(".csv")
}

/// Parse file content based on extension
fn parse_file_content(
    content: &[u8],
    file_name: &str,
) -> Result<Vec<PartialBirthday>, teloxide::RequestError> {
    let result = if file_name.ends_with(".json") {
        parse_json_data(content)
    } else {
        parse_csv_data(content)
    };

    result.map_err(|e| {
        tracing::error!("Failed to parse file: {}", e);
        teloxide::RequestError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to parse file",
        ))
    })
}

/// Parse text content (JSON or CSV)
fn parse_text_content(text: &str) -> Result<Vec<PartialBirthday>, teloxide::RequestError> {
    let result = if is_json_format(text) {
        parse_json_text(text)
    } else {
        parse_csv_text(text)
    };

    result.map_err(|e| {
        tracing::error!("Failed to parse text: {}", e);
        teloxide::RequestError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to parse text",
        ))
    })
}

/// Show batch confirmation dialog
async fn show_batch_confirmation(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    birthdays: Vec<PartialBirthday>,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    let previews: Vec<crate::database::models::BirthdayPreview> =
        birthdays.iter().map(|b| b.to_preview()).collect();

    let next_state = BotState::AddingBatch {
        step: BatchAddStep::Confirmation {
            previews: previews.clone(),
        },
        data: birthdays,
    };

    set_bot_state(&app_state, user_id, chat_id, next_state, 30_i64).await?;

    let confirmation_msg = format_batch_preview(&previews);
    bot.send_message(msg.chat.id, confirmation_msg).await?;
    Ok(())
}

/// Confirm and save batch birthdays
async fn confirm_batch_save(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    chat_id: i64,
) -> Result<(), teloxide::RequestError> {
    let current_state = app_state
        .services
        .conversation_service()
        .get_state(user_id, chat_id)
        .await
        .unwrap_or(None);

    if let Some(BotState::AddingBatch { data, .. }) = current_state {
        let (saved_count, errors) = save_birthdays(&app_state, &data, chat_id, user_id).await;

        clear_bot_state(&app_state, user_id, chat_id).await?;

        let result_msg = format_batch_result(saved_count, errors);
        bot.send_message(msg.chat.id, result_msg)
            .reply_markup(create_main_menu_keyboard())
            .await?;
    }
    Ok(())
}

/// Cancel batch operation
async fn cancel_batch_operation(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    chat_id: i64,
) -> Result<(), teloxide::RequestError> {
    clear_bot_state(&app_state, user_id, chat_id).await?;

    let cancel_msg = crate::handlers::get_user_localized_message(&app_state, user_id, || {
        Messages::operation_cancelled()
    })
    .await;
    bot.send_message(msg.chat.id, cancel_msg)
        .reply_markup(create_main_menu_keyboard())
        .await?;
    Ok(())
}

/// Send invalid input error
async fn send_invalid_input_error(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
) -> Result<(), teloxide::RequestError> {
    let error_msg = crate::handlers::get_user_localized_message(&app_state, user_id, || {
        Messages::invalid_input()
    })
    .await;
    bot.send_message(msg.chat.id, error_msg).await?;
    Ok(())
}

/// Save multiple birthdays and return results
async fn save_birthdays(
    app_state: &AppState,
    data: &[PartialBirthday],
    chat_id: i64,
    user_id: i64,
) -> (usize, Vec<String>) {
    let mut saved_count = 0;
    let mut errors = Vec::new();

    for birthday_data in data {
        if utils::is_birthday_complete(&birthday_data) {
            let input = match (
                &birthday_data.name,
                birthday_data.birth_month,
                birthday_data.birth_day,
            ) {
                (Some(name), Some(month), Some(day)) => {
                    crate::database::models::CreateBirthdayInput {
                        name: name.clone(),
                        birth_month: month,
                        birth_day: day,
                        birth_year: birthday_data.birth_year,
                        username: birthday_data.username.clone(),
                        notes: birthday_data.notes.clone(),
                    }
                }
                _ => {
                    errors.push(format!(
                        "{}: Missing required fields",
                        birthday_data.name.as_deref().unwrap_or("Unknown")
                    ));
                    continue;
                }
            };

            match app_state
                .services
                .birthday_service()
                .add_birthday(chat_id, input, user_id)
                .await
            {
                Ok(_) => saved_count += 1,
                Err(e) => {
                    errors.push(format!(
                        "{}: {}",
                        birthday_data.name.as_deref().unwrap_or("Unknown"),
                        e
                    ));
                }
            }
        }
    }

    (saved_count, errors)
}
