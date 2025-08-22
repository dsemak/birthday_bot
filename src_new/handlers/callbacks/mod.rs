use teloxide::requests::ResponseResult;
use teloxide::{
    prelude::*,
    types::{CallbackQuery, Message},
};

use crate::{localization::Messages, AppState};

pub mod add_batch;
pub mod add_single;
pub mod export;
pub mod list;
pub mod menu;
pub mod search;
pub mod settings;
pub mod stats;

use add_batch::handle_adding_batch;
use add_single::handle_add_single;
use export::handle_export_data;
use list::handle_list_birthdays;
use menu::handle_main_menu;
use search::handle_search_birthdays;
use settings::handle_settings;

/// Handle callback queries from inline keyboards
pub async fn callback_handler(
    bot: Bot,
    query: CallbackQuery,
    app_state: AppState,
) -> ResponseResult<()> {
    let data = query.data.as_deref().unwrap_or("");

    tracing::info!("🔔 Callback query received: '{}'", data);

    // Answer the callback query to remove loading state
    bot.answer_callback_query(query.id.clone()).await?;

    let message = match query.message {
        Some(msg) => msg,
        None => {
            tracing::warn!("No message in callback query");
            return Ok(());
        }
    };

    let user_id = query.from.id.0 as i64;
    let chat_id = message.chat.id.0;

    tracing::info!(
        "Processing callback '{}' from user {} in chat {}",
        data,
        user_id,
        chat_id
    );

    // Check authentication for callbacks that modify data
    let requires_auth = matches!(
        data,
        "add_birthday"
            | "add_single"
            | "add_batch"
            | "list_birthdays"
            | "search_birthdays"
            | "settings"
            | "export_data"
            | "stats"
    );

    if requires_auth {
        tracing::info!("Callback '{}' requires authentication, checking...", data);
        if !super::auth_filter(message.clone(), app_state.clone()).await {
            let user_id = message.from().map(|u| u.id.0 as i64).unwrap_or(0);
            let error_message = if message.chat.is_private() {
                super::get_user_localized_message(&app_state, user_id, || {
                    Messages::no_permission_private()
                })
                .await
            } else {
                super::get_user_localized_message(&app_state, user_id, || {
                    Messages::no_permission_group()
                })
                .await
            };

            tracing::warn!(
                "Authentication failed for user {} in chat {}",
                user_id,
                chat_id
            );
            bot.answer_callback_query(&query.id)
                .text(error_message)
                .await?;
            return Ok(());
        }
        tracing::info!(
            "Authentication passed for user {} in chat {}",
            user_id,
            chat_id
        );
    }

    tracing::info!("Routing callback '{}' to appropriate handler", data);

    match data {
        "add_birthday" => {
            tracing::info!("Handling add_birthday callback");
            handle_add_birthday(bot, message, app_state, user_id, chat_id).await?
        }
        "add_single" => {
            tracing::info!("Handling add_single callback");
            handle_add_single(bot, message, app_state, user_id, chat_id).await?
        }
        "add_batch" => {
            tracing::info!("Handling add_batch callback");
            handle_adding_batch(bot, message, app_state, user_id, chat_id).await?
        }
        "list_birthdays" => {
            tracing::info!("Handling list_birthdays callback");
            handle_list_birthdays(bot, message, app_state, user_id).await?
        }
        "search_birthdays" => {
            tracing::info!("Handling search_birthdays callback");
            handle_search_birthdays(bot, message, app_state, user_id, chat_id).await?
        }
        "settings" => {
            tracing::info!("Handling settings callback");
            handle_settings(bot, message, app_state, user_id, chat_id).await?
        }
        "export_data" => {
            tracing::info!("Handling export_data callback");
            handle_export_data(bot, message, app_state, user_id, chat_id).await?
        }
        "stats" => {
            tracing::info!("Handling stats callback");
            stats::handle_stats_callback(bot, message, app_state, user_id, chat_id).await?
        }
        "main_menu" => {
            tracing::info!("Handling main_menu callback");
            handle_main_menu(bot, message, app_state, user_id, chat_id).await?
        }
        _ => {
            tracing::warn!("Unknown callback data: '{}'", data);
        }
    }

    tracing::info!("Callback '{}' handled successfully", data);
    Ok(())
}

/// Helper function to get localized text with fallback
async fn get_localized_text_with_fallback<F>(
    app_state: &AppState,
    user_id: i64,
    message_fn: F,
) -> String
where
    F: Fn() -> crate::localization::LocalizedText,
{
    super::get_user_localized_message(app_state, user_id, message_fn).await
}

/// Handle "Add Birthday" callback
async fn handle_add_birthday(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    user_id: i64,
    _chat_id: i64,
) -> ResponseResult<()> {
    // Get localized text for user using helper function
    let text = get_localized_text_with_fallback(&app_state, user_id, || {
        Messages::add_birthday_choice_title()
    })
    .await;

    let add_single_text = get_localized_text_with_fallback(&app_state, user_id, || {
        Messages::add_single_choice_button()
    })
    .await;

    let add_batch_text = get_localized_text_with_fallback(&app_state, user_id, || {
        Messages::add_batch_choice_button()
    })
    .await;

    let back_text =
        get_localized_text_with_fallback(&app_state, user_id, || Messages::back_button()).await;

    let keyboard = teloxide::types::InlineKeyboardMarkup::new(vec![
        vec![
            teloxide::types::InlineKeyboardButton::callback(add_single_text, "add_single"),
            teloxide::types::InlineKeyboardButton::callback(add_batch_text, "add_batch"),
        ],
        vec![teloxide::types::InlineKeyboardButton::callback(
            back_text,
            "main_menu",
        )],
    ]);

    bot.edit_message_text(msg.chat.id, msg.id, text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

/// Create a main menu button keyboard
pub fn create_main_menu_keyboard() -> teloxide::types::InlineKeyboardMarkup {
    teloxide::types::InlineKeyboardMarkup::new([[teloxide::types::InlineKeyboardButton::callback(
        Messages::main_menu_button().get(crate::localization::Language::default()),
        "main_menu",
    )]])
}
