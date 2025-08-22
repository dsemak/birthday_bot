/// Parses a date string in the format DD.MM, DD-MM, DD/MM, or DD.MM.YYYY, DD-MM-YYYY, DD/MM/YYYY.
///
/// Returns (day, month, optional_year) if valid, otherwise None.
pub fn parse_date_with_year(date_str: &str) -> Option<(u8, u8, Option<u16>)> {
    // Split by any of the allowed delimiters and collect parts
    let parts: Vec<_> = date_str
        .split(|c| c == '.' || c == '-' || c == '/')
        .map(str::trim)
        .collect();

    match parts.as_slice() {
        [day_str, month_str] | [day_str, month_str, ""] => {
            let day = day_str.parse::<u8>().ok()?;
            let month = month_str.parse::<u8>().ok()?;
            if !is_valid_date(day, month) {
                return None;
            }
            Some((day, month, None))
        }
        [day_str, month_str, year_str] => {
            let day = day_str.parse::<u8>().ok()?;
            let month = month_str.parse::<u8>().ok()?;
            let year = year_str.parse::<u16>().ok()?;
            if !is_valid_date(day, month) || !is_valid_year(year) {
                return None;
            }
            Some((day, month, Some(year)))
        }
        _ => None,
    }
}

/// Parses a date string in the format DD.MM, DD-MM, or DD/MM.
///
/// Returns (day, month) if valid, otherwise None.
pub fn parse_date(date_str: &str) -> Option<(u8, u8)> {
    parse_date_with_year(date_str).map(|(day, month, _)| (day, month))
}

/// Checks if the given day and month form a valid calendar date (ignoring leap years).
pub fn is_valid_date(day: u8, month: u8) -> bool {
    const DAYS_IN_MONTH: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month_idx = month.checked_sub(1).unwrap_or(0) as usize;
    let max_day = DAYS_IN_MONTH.get(month_idx).unwrap();
    (1..=*max_day).contains(&day)
}

/// Checks if the year is within the allowed range (1900..=2100).
pub fn is_valid_year(year: u16) -> bool {
    (1900..=2100).contains(&year)
}

/// Formats a date as "DD.MM.YYYY" if year is present, otherwise as "DD.MM".
pub fn format_date(day: u8, month: u8, year: Option<u16>) -> String {
    match year {
        Some(y) => format!("{:02}.{:02}.{}", day, month, y),
        None => format!("{:02}.{:02}", day, month),
    }
}
