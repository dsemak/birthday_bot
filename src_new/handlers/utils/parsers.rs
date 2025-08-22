use crate::database::models::PartialBirthday;
use crate::errors::BotError;

/// Parse JSON data into birthdays
pub fn parse_json_data(data: &[u8]) -> Result<Vec<PartialBirthday>, BotError> {
    let content_str = std::str::from_utf8(data)
        .map_err(|e| BotError::invalid_file_format(format!("Invalid UTF-8: {}", e)))?;

    parse_json_text(content_str)
}

/// Parse JSON text into birthdays
pub fn parse_json_text(text: &str) -> Result<Vec<PartialBirthday>, BotError> {
    let json_value: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| BotError::invalid_file_format(format!("Invalid JSON: {}", e)))?;

    let birthdays = match json_value {
        serde_json::Value::Array(arr) => arr
            .into_iter()
            .filter_map(|item| serde_json::from_value::<PartialBirthday>(item).ok())
            .collect(),
        serde_json::Value::Object(obj) => {
            if let Ok(birthday) =
                serde_json::from_value::<PartialBirthday>(serde_json::Value::Object(obj))
            {
                vec![birthday]
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    };

    Ok(birthdays)
}

/// Parse CSV data into birthdays
pub fn parse_csv_data(data: &[u8]) -> Result<Vec<PartialBirthday>, BotError> {
    let content_str = std::str::from_utf8(data)
        .map_err(|e| BotError::invalid_file_format(format!("Invalid UTF-8: {}", e)))?;

    parse_csv_text(content_str)
}

/// Parse CSV text into birthdays
pub fn parse_csv_text(text: &str) -> Result<Vec<PartialBirthday>, BotError> {
    let mut birthdays = Vec::new();
    let lines: Vec<&str> = text.lines().collect();

    if lines.is_empty() {
        return Ok(birthdays);
    }

    // Skip header if present
    let start_line = if lines
        .first()
        .map(|l| l.contains("name") || l.contains("имя"))
        .unwrap_or(false)
    {
        1
    } else {
        0
    };

    for line in lines.iter().skip(start_line) {
        if let Some(birthday) = parse_csv_line(line) {
            birthdays.push(birthday);
        }
    }

    Ok(birthdays)
}

/// Parse single CSV line
fn parse_csv_line(line: &str) -> Option<PartialBirthday> {
    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

    if parts.len() < 2 {
        return None;
    }

    let name = parts[0].to_string();
    if name.is_empty() {
        return None;
    }

    let (birth_day, birth_month, birth_year) = parse_date_parts(parts[1])?;

    let mut birthday = PartialBirthday {
        name: Some(name),
        birth_day: Some(birth_day),
        birth_month: Some(birth_month),
        birth_year,
        username: None,
        notes: None,
    };

    // Parse username if present
    if parts.len() >= 3 && !parts[2].is_empty() {
        let username = parts[2].trim_start_matches('@').to_string();
        birthday.username = Some(username);
    }

    // Parse notes if present
    if parts.len() >= 4 && !parts[3].is_empty() {
        birthday.notes = Some(parts[3].to_string());
    }

    Some(birthday)
}

/// Parse date parts from string
fn parse_date_parts(date_str: &str) -> Option<(u8, u8, Option<u16>)> {
    crate::utils::parse_date_with_year(date_str)
}

/// Determine if text looks like JSON
pub fn is_json_format(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.starts_with('[') || trimmed.starts_with('{')
}
