// Utility modules
pub mod date_utils;
pub mod validation;

// Re-export commonly used functions
pub use date_utils::*;
pub use validation::*;

/// Escape text for MarkdownV2 format
/// This function escapes special characters that need to be escaped in MarkdownV2
pub fn escape_markdown_v2(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|'
            | '{' | '}' | '.' | '!' => {
                format!("\\{}", c)
            }
            _ => c.to_string(),
        })
        .collect()
}

/// Format birthday list for MarkdownV2
/// This function should be replaced with localized version
pub fn format_birthday_list(birthdays: &[crate::database::models::Birthday]) -> String {
    if birthdays.is_empty() {
        return "📋 *Birthday List*\n\n❌ No birthdays added yet\\.\n\nTry adding your first birthday using the \\\"➕ Add Birthday\\\" button or use `/test` command to add test data\\.".to_string();
    }

    let mut text = "📋 *Birthday List*\n\n".to_string();

    for (i, birthday) in birthdays.iter().enumerate() {
        let username_part = if let Some(username) = &birthday.username {
            format!(" \\(@{}\\)", escape_markdown_v2(username))
        } else {
            String::new()
        };

        text.push_str(&format!(
            "{}\\. {} \\- {}{}\n",
            i + 1,
            escape_markdown_v2(&birthday.name),
            birthday.format_date(),
            username_part
        ));
    }

    if birthdays.len() == 10 {
        text.push_str(
            "\n_Showing first 10 records_\\. Add more using the \\\"➕ Add Birthday\\\" button\\.",
        );
    }

    text
}

/// Format search results for MarkdownV2
/// This function should be replaced with localized version
pub fn format_search_results(
    query: &str,
    birthdays: &[crate::database::models::Birthday],
) -> String {
    if birthdays.is_empty() {
        return format!(
            "🔍 *Search: \\\"{}\\\"*\n\n❌ Nothing found\\.\n\nTry a different query or add a birthday using the \\\"➕ Add Birthday\\\" button\\.",
            escape_markdown_v2(query)
        );
    }

    let mut text = format!(
        "🔍 *Search: \\\"{}\\\"*\n\nFound {} records:\n\n",
        escape_markdown_v2(query),
        birthdays.len()
    );

    for (i, birthday) in birthdays.iter().enumerate() {
        let username_part = if let Some(username) = &birthday.username {
            format!(" \\(@{}\\)", escape_markdown_v2(username))
        } else {
            String::new()
        };

        text.push_str(&format!(
            "{}\\. {} \\- {}{}\n",
            i + 1,
            escape_markdown_v2(&birthday.name),
            birthday.format_date(),
            username_part
        ));
    }

    text
}

/// Format error message for MarkdownV2 using localization
/// This function should be replaced with localized version
pub fn format_error_message(error: &str) -> String {
    // For now, return English version as fallback
    // This should be replaced with proper localization
    format!("❌ Error: {}", error)
}

/// Format localized error message with user language support
pub async fn format_error_message_localized(
    error: &str,
    user_id: i64,
    localization_service: &crate::services::LocalizationService,
) -> Result<String, anyhow::Error> {
    let language = localization_service.get_user_language(user_id).await?;
    let localized = crate::localization::Messages::error_with_details(error);
    Ok(localized.get(language).to_string())
}
