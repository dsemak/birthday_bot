use teloxide::prelude::*;
use teloxide::types::ChatId;

use crate::database::models::{AddStep, BotState, PartialBirthday};
use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_message;
use crate::handlers::utils::*;
use crate::localization::Messages;
use crate::utils;
use crate::AppState;

use super::send_error_message;

/// Handle adding single birthday step by step
pub async fn handle_adding_single(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    step: AddStep,
    mut data: PartialBirthday,
    text: &str,
) -> ResponseResult<()> {
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or_default();
    let chat_id = msg.chat.id.0;

    // Get user language or default to English if error
    let language = app_state
        .services
        .localization_service()
        .get_user_language(user_id)
        .await
        .unwrap_or_default();

    match step {
        AddStep::Name => {
            data.name = Some(text.trim().to_string());
            if let Err(e) = data.validate(language) {
                send_error_message(&bot, &msg, &app_state, user_id, || {
                    Messages::validation_error(&e.to_string())
                })
                .await?;
                return Ok(());
            }
            next_step(
                &app_state,
                &bot,
                chat_id,
                user_id,
                &data,
                AddStep::Date,
                Messages::add_single_enter_day().get(language).to_string(),
            )
            .await?;
        }
        AddStep::Date => {
            let date_str = text.trim();
            match utils::parse_date(date_str) {
                Some((day, month)) => {
                    data.birth_day = Some(day);
                    data.birth_month = Some(month);
                    next_step(
                        &app_state,
                        &bot,
                        chat_id,
                        user_id,
                        &data,
                        AddStep::Year,
                        Messages::add_single_enter_year().get(language).to_string(),
                    )
                    .await?;
                }
                None => {
                    send_error_message(&bot, &msg, &app_state, user_id, || {
                        Messages::format_invalid_date()
                    })
                    .await?;
                }
            }
        }
        AddStep::Year => {
            let year_str = text.trim();
            let skip =
                year_str.is_empty() || matches!(year_str.to_lowercase().as_str(), "нет" | "no");
            if skip {
                next_step(
                    &app_state,
                    &bot,
                    chat_id,
                    user_id,
                    &data,
                    AddStep::Username,
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::add_single_enter_username()
                    })
                    .await,
                )
                .await?;
            } else {
                match year_str.parse::<u16>() {
                    Ok(year) if (1900..=2100).contains(&year) => {
                        data.birth_year = Some(year);
                        next_step(
                            &app_state,
                            &bot,
                            chat_id,
                            user_id,
                            &data,
                            AddStep::Username,
                            get_user_localized_message(&app_state, user_id, || {
                                Messages::add_single_enter_username()
                            })
                            .await,
                        )
                        .await?;
                    }
                    Ok(_) => {
                        send_error_message(&bot, &msg, &app_state, user_id, || {
                            Messages::validation_year_range()
                        })
                        .await?;
                    }
                    Err(_) => {
                        send_error_message(&bot, &msg, &app_state, user_id, || {
                            Messages::format_invalid_year()
                        })
                        .await?;
                    }
                }
            }
        }
        AddStep::Username => {
            if let Err(e) = utils::validate_username(text, language) {
                send_error_message(&bot, &msg, &app_state, user_id, || {
                    Messages::validation_error(&e.to_string())
                })
                .await?;
                return Ok(());
            }
            data.username = if text.trim().is_empty() {
                None
            } else {
                Some(text.trim().to_string())
            };
            next_step(
                &app_state,
                &bot,
                chat_id,
                user_id,
                &data,
                AddStep::Notes,
                Messages::add_single_enter_notes().get(language).to_string(),
            )
            .await?;
        }
        AddStep::Notes => {
            if let Err(e) = utils::validate_notes(text, language) {
                send_error_message(&bot, &msg, &app_state, user_id, || {
                    Messages::validation_error(&e.to_string())
                })
                .await?;
                return Ok(());
            }
            data.notes = if text.trim().is_empty() {
                None
            } else {
                Some(text.trim().to_string())
            };
            let preview = data.to_preview();
            let confirmation_msg = format_birthday_preview(&preview);

            next_step(
                &app_state,
                &bot,
                chat_id,
                user_id,
                &data,
                AddStep::Confirmation,
                confirmation_msg,
            )
            .await?;
        }
        AddStep::Confirmation => {
            handle_confirmation_response(
                text,
                confirm_single_save(
                    bot.clone(),
                    msg.clone(),
                    app_state.clone(),
                    data,
                    user_id,
                    chat_id,
                ),
                cancel_single_operation(
                    bot.clone(),
                    msg.clone(),
                    app_state.clone(),
                    user_id,
                    chat_id,
                ),
                send_invalid_input_error(bot, msg, app_state, user_id),
            )
            .await?;
        }
    }

    Ok(())
}

/// Confirm and save single birthday
///
/// Saves the birthday and sends a success or error message.
async fn confirm_single_save(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    data: PartialBirthday,
    user_id: i64,
    chat_id: i64,
) -> Result<(), teloxide::RequestError> {
    let input = match (&data.name, data.birth_month, data.birth_day) {
        (Some(name), Some(month), Some(day)) => crate::database::models::CreateBirthdayInput {
            name: name.clone(),
            birth_month: month,
            birth_day: day,
            birth_year: data.birth_year,
            username: data.username,
            notes: data.notes,
        },
        _ => {
            send_error_message(&bot, &msg, &app_state, user_id, || {
                Messages::birthday_save_error()
            })
            .await?;
            return Ok(());
        }
    };

    let result = app_state
        .services
        .birthday_service()
        .add_birthday(chat_id, input, user_id)
        .await;

    clear_bot_state(&app_state, user_id, chat_id).await?;

    let (msg_text, _is_success) = match result {
        Ok(_) => (
            crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::add_single_success()
            })
            .await,
            true,
        ),
        Err(_) => (
            crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::birthday_save_error()
            })
            .await,
            false,
        ),
    };

    bot.send_message(msg.chat.id, msg_text)
        .reply_markup(create_main_menu_keyboard())
        .await?;

    Ok(())
}

// Helper function to move to the next step
async fn next_step(
    app_state: &AppState,
    bot: &Bot,
    chat_id: i64,
    user_id: i64,
    data: &PartialBirthday,
    step: AddStep,
    prompt: String,
) -> ResponseResult<()> {
    let next_state = BotState::AddingSingle {
        step,
        data: data.clone(),
    };
    set_bot_state(app_state, user_id, chat_id, next_state, 30_i64).await?;
    bot.send_message(ChatId(chat_id), prompt).await?;
    Ok(())
}

/// Cancel single birthday operation
///
/// Clears state and notifies user about cancellation.
async fn cancel_single_operation(
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
///
/// Sends a generic invalid input message.
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
