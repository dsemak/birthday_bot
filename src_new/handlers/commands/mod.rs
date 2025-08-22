mod cancel;
mod help;
mod menu;
mod start;
mod test;

use teloxide::{prelude::*, utils::command::BotCommands};

use crate::localization::Messages;
use crate::AppState;

use super::{auth_filter, get_user_localized_message};

/// Bot commands
#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Show this message")]
    Help,
    #[command(description = "Start working with the bot")]
    Start,
    #[command(description = "Open main menu")]
    Menu,
    #[command(description = "Show statistics")]
    Stats,
    #[command(description = "Cancel current operation")]
    Cancel,
    #[command(description = "Test database")]
    Test,
}

/// Handle bot commands
pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    app_state: AppState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    let user = match msg.from() {
        Some(user) => user,
        None => {
            tracing::error!("No user in message");
            return Ok(());
        }
    };

    // Log command usage
    tracing::info!(
        "Command {:?} from user {} in chat {}",
        cmd,
        user.id,
        chat_id
    );

    // Check authentication for commands that modify data
    match cmd {
        Command::Start => start::handle_start(bot, msg, app_state).await?,
        Command::Help => help::handle_help(bot, msg, app_state).await?,
        Command::Test => test::handle_test(bot, msg, app_state).await?,
        Command::Menu => {
            // Check auth for menu access
            if !auth_filter(msg.clone(), app_state.clone()).await {
                let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
                let error_message = if msg.chat.is_private() {
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::no_permission_private()
                    })
                    .await
                } else {
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::no_permission_group()
                    })
                    .await
                };
                bot.send_message(msg.chat.id, error_message).await?;
                return Ok(());
            }
            menu::handle_menu(bot, msg, app_state).await?
        }
        Command::Stats => {
            // Check auth for stats access
            if !auth_filter(msg.clone(), app_state.clone()).await {
                let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
                let error_message = if msg.chat.is_private() {
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::no_permission_private()
                    })
                    .await
                } else {
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::no_permission_stats()
                    })
                    .await
                };
                bot.send_message(msg.chat.id, error_message).await?;
                return Ok(());
            }
            super::callbacks::stats::handle_stats_command(bot, msg, app_state).await?
        }
        Command::Cancel => {
            // Check auth for cancel access
            if !auth_filter(msg.clone(), app_state.clone()).await {
                let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);
                let error_message = if msg.chat.is_private() {
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::no_permission_private()
                    })
                    .await
                } else {
                    get_user_localized_message(&app_state, user_id, || {
                        Messages::no_permission_cancel()
                    })
                    .await
                };
                bot.send_message(msg.chat.id, error_message).await?;
                return Ok(());
            }
            cancel::handle_cancel(bot, msg, app_state).await?
        }
    }

    Ok(())
}

/// Create main menu keyboard
pub(crate) fn create_main_menu_keyboard() -> teloxide::types::InlineKeyboardMarkup {
    use crate::localization::{Language, Messages};
    use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(
                Messages::add_single_button().get(Language::default()),
                "add_birthday",
            ),
            InlineKeyboardButton::callback(
                Messages::list_button().get(Language::default()),
                "list_birthdays",
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                Messages::search_button().get(Language::default()),
                "search_birthdays",
            ),
            InlineKeyboardButton::callback(
                Messages::settings_button().get(Language::default()),
                "settings",
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                Messages::export_button().get(Language::default()),
                "export_data",
            ),
            InlineKeyboardButton::callback(
                Messages::stats_button().get(Language::default()),
                "stats",
            ),
        ],
    ])
}
