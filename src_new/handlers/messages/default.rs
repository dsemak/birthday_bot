use teloxide::prelude::*;

use crate::database::models::PartialBirthday;
use crate::handlers::commands::create_main_menu_keyboard;
use crate::handlers::get_user_localized_message;
use crate::localization::Messages;
use crate::utils;
use crate::AppState;

/// Helper function to convert BotError to RequestError
fn convert_bot_error(e: crate::errors::BotError) -> teloxide::RequestError {
    tracing::error!("Bot error: {}", e);
    teloxide::RequestError::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Bot operation failed",
    ))
}

/// Handle default messages (auto-detect birthday format or search)
pub async fn handle_default_message(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    text: &str,
) -> ResponseResult<()> {
    let user_id = msg.from().unwrap().id.0 as i64;
    let chat_id = msg.chat.id.0;

    // Try to parse as birthday first
    if let Some(birthday_data) = parse_birthday_message(text) {
        handle_birthday_parsing(bot, msg, app_state, birthday_data, user_id, chat_id).await?;
    } else {
        // Try to search for birthdays
        handle_search_message(bot, msg, app_state, text, user_id, chat_id).await?;
    }

    Ok(())
}

/// Parse birthday from message text
fn parse_birthday_message(text: &str) -> Option<PartialBirthday> {
    let text = text.trim();

    // Pattern 1: "Имя ДД.ММ.ГГГГ @username заметки"
    // Pattern 2: "Имя ДД.ММ @username заметки"
    // Pattern 3: "Имя ДД-ММ-ГГГГ @username заметки"
    // Pattern 4: "Имя ДД/ММ/ГГГГ @username заметки"

    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    // Extract name (first part)
    let name = parts[0].to_string();

    // Find date part
    let mut date_part = None;
    let mut username = None;
    let mut notes_parts = Vec::new();

    for (_, part) in parts.iter().enumerate().skip(1) {
        if date_part.is_none() && is_date_format(part) {
            date_part = Some(*part);
        } else if username.is_none() && part.starts_with('@') {
            username = Some(part[1..].to_string());
        } else if date_part.is_some() {
            // Everything after date is notes
            notes_parts.push(*part);
        }
    }

    let date_part = date_part?;
    let (day, month, year) = parse_date_with_year(date_part)?;

    let notes = if notes_parts.is_empty() {
        None
    } else {
        Some(notes_parts.join(" "))
    };

    Some(PartialBirthday {
        name: Some(name),
        birth_day: Some(day),
        birth_month: Some(month),
        birth_year: year,
        username,
        notes,
    })
}

/// Check if string looks like a date
fn is_date_format(s: &str) -> bool {
    let separators = ['.', '-', '/'];
    let parts: Vec<&str> = s.split(&separators[..]).collect();

    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }

    // Check if parts are numeric
    for part in &parts {
        if !part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }

    true
}

/// Parse date with optional year
fn parse_date_with_year(date_str: &str) -> Option<(u8, u8, Option<u16>)> {
    crate::utils::parse_date_with_year(date_str)
}

/// Handle birthday parsing result
async fn handle_birthday_parsing(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    birthday_data: PartialBirthday,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    // Validate the birthday data
    let errors = birthday_data.validate();
    let preview = birthday_data.to_preview();

    if !errors.is_empty() {
        let fix_errors_msg = get_user_localized_message(&app_state, user_id, || {
            Messages::fix_errors_and_try_again()
        })
        .await;
        let error_msg = format_birthday_errors(&preview, &errors, &fix_errors_msg);
        bot.send_message(msg.chat.id, error_msg).await?;
        return Ok(());
    }

    // Show confirmation
    let confirmation_msg = format_birthday_confirmation(&preview);
    bot.send_message(msg.chat.id, confirmation_msg).await?;

    // Save birthday
    let input = crate::database::models::CreateBirthdayInput {
        name: birthday_data.name.unwrap(),
        birth_month: birthday_data.birth_month.unwrap(),
        birth_day: birthday_data.birth_day.unwrap(),
        birth_year: birthday_data.birth_year,
        username: birthday_data.username,
        notes: birthday_data.notes,
    };
    match app_state
        .services
        .birthday_service()
        .add_birthday(chat_id, input, user_id)
        .await
    {
        Ok(_) => {
            let success_msg =
                get_user_localized_message(&app_state, user_id, || Messages::add_single_success())
                    .await;
            bot.send_message(msg.chat.id, success_msg)
                .reply_markup(create_main_menu_keyboard())
                .await?;
        }
        Err(e) => {
            let error_msg =
                get_user_localized_message(&app_state, user_id, || Messages::birthday_save_error())
                    .await;
            bot.send_message(msg.chat.id, error_msg)
                .reply_markup(create_main_menu_keyboard())
                .await?;
        }
    }

    Ok(())
}

/// Handle search message
async fn handle_search_message(
    bot: Bot,
    msg: Message,
    app_state: AppState,
    search_query: &str,
    user_id: i64,
    chat_id: i64,
) -> ResponseResult<()> {
    let search_query = search_query.trim();
    if search_query.is_empty() {
        let help_msg =
            get_user_localized_message(&app_state, user_id, || Messages::search_instructions())
                .await;
        bot.send_message(msg.chat.id, help_msg)
            .reply_markup(create_main_menu_keyboard())
            .await?;
        return Ok(());
    }

    // Search for birthdays
    let filter = crate::database::models::BirthdayFilter {
        chat_id: Some(chat_id),
        name: Some(search_query.to_string()),
        limit: Some(10),
        ..Default::default()
    };

    match app_state
        .services
        .birthday_service()
        .get_birthdays(filter)
        .await
    {
        Ok(birthdays) => {
            if birthdays.is_empty() {
                let not_found_msg = get_user_localized_message(&app_state, user_id, || {
                    Messages::search_not_found(search_query)
                })
                .await;
                bot.send_message(msg.chat.id, not_found_msg)
                    .reply_markup(create_main_menu_keyboard())
                    .await?;
            } else {
                let found_count_msg = get_user_localized_message(&app_state, user_id, || {
                    Messages::search_found_count(birthdays.len())
                })
                .await;
                let results_msg = format_search_results(&birthdays, search_query, &found_count_msg);
                bot.send_message(msg.chat.id, results_msg)
                    .reply_markup(create_main_menu_keyboard())
                    .await?;
            }
        }
        Err(e) => {
            let error_msg =
                get_user_localized_message(&app_state, user_id, || Messages::search_error()).await;
            bot.send_message(msg.chat.id, error_msg)
                .reply_markup(create_main_menu_keyboard())
                .await?;
        }
    }

    Ok(())
}

/// Format birthday errors
fn format_birthday_errors(
    preview: &crate::database::models::BirthdayPreview,
    errors: &[String],
    fix_errors_msg: &str,
) -> String {
    let mut lines = vec![
        "❌ **Ошибки в данных дня рождения:**".to_string(),
        "".to_string(),
    ];

    lines.push(format!("👤 **Имя:** {}", preview.name));

    let date_str = if let Some(year) = preview.birth_year {
        format!(
            "{:02}.{:02}.{}",
            preview.birth_day, preview.birth_month, year
        )
    } else {
        format!("{:02}.{:02}", preview.birth_day, preview.birth_month)
    };
    lines.push(format!("📅 **Дата:** {}", date_str));

    if let Some(ref username) = preview.username {
        lines.push(format!("🔗 **Username:** @{}", username));
    }

    if let Some(ref notes) = preview.notes {
        lines.push(format!("📝 **Заметки:** {}", notes));
    }

    lines.push("".to_string());
    lines.push("⚠️ **Ошибки:**".to_string());
    for error in errors {
        lines.push(format!("• {}", error));
    }

    lines.push("".to_string());
    lines.push(fix_errors_msg.to_string());

    lines.join("\n")
}

/// Format birthday confirmation
fn format_birthday_confirmation(preview: &crate::database::models::BirthdayPreview) -> String {
    let mut lines = vec![
        "✅ **День рождения распознан автоматически:**".to_string(),
        "".to_string(),
    ];

    lines.push(format!("👤 **Имя:** {}", preview.name));

    let date_str = if let Some(year) = preview.birth_year {
        format!(
            "{:02}.{:02}.{}",
            preview.birth_day, preview.birth_month, year
        )
    } else {
        format!("{:02}.{:02}", preview.birth_day, preview.birth_month)
    };
    lines.push(format!("📅 **Дата:** {}", date_str));

    if let Some(ref username) = preview.username {
        lines.push(format!("🔗 **Username:** @{}", username));
    }

    if let Some(ref notes) = preview.notes {
        lines.push(format!("📝 **Заметки:** {}", notes));
    }

    lines.push("".to_string());
    lines.push("Сохранение...".to_string());

    lines.join("\n")
}

/// Format search results
fn format_search_results(
    birthdays: &[crate::database::models::Birthday],
    query: &str,
    found_count_msg: &str,
) -> String {
    let mut lines = vec![
        format!("🔍 **Результаты поиска для '{}':**", query),
        "".to_string(),
    ];

    for (i, birthday) in birthdays.iter().enumerate() {
        lines.push(format!("{}. **{}**", i + 1, birthday.name));

        let date_str = if let Some(year) = birthday.birth_year {
            format!(
                "{:02}.{:02}.{}",
                birthday.birth_day, birthday.birth_month, year
            )
        } else {
            format!("{:02}.{:02}", birthday.birth_day, birthday.birth_month)
        };
        lines.push(format!("   📅 {}", date_str));

        if let Some(ref username) = birthday.username {
            lines.push(format!("   🔗 @{}", username));
        }

        if let Some(ref notes) = birthday.notes {
            lines.push(format!("   📝 {}", notes));
        }

        lines.push("".to_string());
    }

    lines.push(found_count_msg.to_string());

    lines.join("\n")
}
