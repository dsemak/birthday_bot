use crate::database::models::PartialBirthday;
use crate::errors::{BotError, BotResult};
use crate::localization;

/// Validates username format and length
pub fn validate_username(username: &str, language: localization::Language) -> BotResult<()> {
    if username.is_empty() {
        // Username is optional
        return Ok(());
    }

    let clean_username = if username.starts_with('@') {
        &username[1..]
    } else {
        username
    };

    if clean_username.is_empty() {
        return Err(BotError::validation(
            localization::Messages::validation_username_empty().get(language),
        ));
    }

    if clean_username.len() > 100 {
        return Err(BotError::validation(
            localization::Messages::validation_name_too_long().get(language),
        ));
    }

    // Check for valid characters (letters, numbers, underscores)
    if !clean_username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(BotError::validation(
            localization::Messages::validation_username_invalid().get(language),
        ));
    }

    Ok(())
}

/// Validates notes length
pub fn validate_notes(notes: &str, language: localization::Language) -> BotResult<()> {
    if notes.len() > 500 {
        return Err(BotError::validation(
            localization::Messages::validation_notes_too_long().get(language),
        ));
    }
    Ok(())
}

/// Validates name length and format
pub fn validate_name(name: &str, language: localization::Language) -> BotResult<()> {
    if name.trim().is_empty() {
        return Err(BotError::validation(
            localization::Messages::validation_name_empty().get(language),
        ));
    }

    if name.len() > 100 {
        return Err(BotError::validation(
            localization::Messages::validation_name_too_long().get(language),
        ));
    }

    Ok(())
}

/// Validates a complete birthday entry
pub fn validate_birthday(
    birthday: &PartialBirthday,
    language: localization::Language,
) -> BotResult<()> {
    // Validate name
    if let Some(name) = &birthday.name {
        validate_name(name, language)?;
    }

    // Validate date
    if let (Some(day), Some(month)) = (birthday.birth_day, birthday.birth_month) {
        if !crate::utils::is_valid_date(day, month) {
            return Err(BotError::validation(
                localization::Messages::validation_invalid_date().get(language),
            ));
        }
    } else {
        return Err(BotError::validation(
            localization::Messages::validation_date_required().get(language),
        ));
    }

    // Validate year if present
    if let Some(year) = birthday.birth_year {
        if !crate::utils::is_valid_year(year) {
            return Err(BotError::validation(
                localization::Messages::validation_year_range().get(language),
            ));
        }
    }

    // Validate username if present
    if let Some(ref username) = birthday.username {
        validate_username(username, language)?;
    }

    // Validate notes if present
    if let Some(ref notes) = birthday.notes {
        validate_notes(notes, language)?;
    }

    Ok(())
}

/// Validates if a birthday entry is complete (has all required fields)
pub fn is_birthday_complete(birthday: &PartialBirthday) -> bool {
    birthday.name.is_some() && birthday.birth_day.is_some() && birthday.birth_month.is_some()
}
