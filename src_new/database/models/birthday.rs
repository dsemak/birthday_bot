use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Birthday record in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Birthday {
    pub id: Uuid,
    pub chat_id: i64,
    pub name: String,
    pub birth_month: i16,
    pub birth_day: i16,
    pub birth_year: Option<i16>,
    pub username: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Input data for creating a new birthday
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct CreateBirthdayInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(range(min = 1, max = 12, message = "Month must be between 1 and 12"))]
    pub birth_month: u8,

    #[validate(range(min = 1, max = 31, message = "Day must be between 1 and 31"))]
    pub birth_day: u8,

    #[validate(range(min = 1900, max = 2100, message = "Year must be between 1900 and 2100"))]
    pub birth_year: Option<u16>,

    #[validate(length(max = 100, message = "Username must be at most 100 characters"))]
    pub username: Option<String>,

    #[validate(length(max = 500, message = "Notes must be at most 500 characters"))]
    pub notes: Option<String>,
}

/// Input data for updating an existing birthday
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct UpdateBirthdayInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,

    #[validate(range(min = 1, max = 12, message = "Month must be between 1 and 12"))]
    pub birth_month: Option<u8>,

    #[validate(range(min = 1, max = 31, message = "Day must be between 1 and 31"))]
    pub birth_day: Option<u8>,

    #[validate(range(min = 1900, max = 2100, message = "Year must be between 1900 and 2100"))]
    pub birth_year: Option<Option<u16>>,

    #[validate(length(max = 100, message = "Username must be at most 100 characters"))]
    pub username: Option<Option<String>>,

    #[validate(length(max = 500, message = "Notes must be at most 500 characters"))]
    pub notes: Option<Option<String>>,
}

/// Batch input for creating multiple birthdays
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct BatchCreateBirthdayInput {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Must provide between 1 and 100 birthdays"
    ))]
    pub birthdays: Vec<CreateBirthdayInput>,
}

/// Filter for querying birthdays
#[derive(Debug, Clone, Default)]
pub struct BirthdayFilter {
    pub chat_id: Option<i64>,
    pub name: Option<String>,
    pub birth_month: Option<u8>,
    pub birth_day: Option<u8>,
    pub created_by: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Birthday with additional computed fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthdayWithAge {
    #[serde(flatten)]
    pub birthday: Birthday,
    pub age: Option<u16>,
    pub days_until_birthday: i32,
}

/// Statistics about birthdays in a chat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthdayStats {
    pub total_birthdays: i64,
    pub birthdays_today: i64,
    pub birthdays_with_year: i64,
    pub upcoming_birthdays: i64,
}

impl Birthday {
    /// Format the birthday date as DD-MM
    pub fn format_date(&self) -> String {
        format!("{:02}-{:02}", self.birth_day, self.birth_month)
    }

    /// Format the birthday date with year as DD-MM-YYYY
    pub fn format_date_with_year(&self) -> String {
        match self.birth_year {
            Some(year) => format!("{:02}-{:02}-{}", self.birth_day, self.birth_month, year),
            None => self.format_date(),
        }
    }

    /// Get the username formatted for display
    /// Format the username for display (with leading '@' if present)
    pub fn format_username(&self) -> String {
        self.username
            .as_ref()
            .filter(|u| !u.is_empty())
            .map(|u| {
                if u.starts_with('@') {
                    u.clone()
                } else {
                    format!("@{}", u)
                }
            })
            .unwrap_or_default()
    }

    /// Calculate age if birth year is available
    pub fn calculate_age(&self) -> Option<u16> {
        self.birth_year.map(|birth_year| {
            let current_year = Utc::now().year() as u16;
            let current_month = Utc::now().month() as u8;
            let current_day = Utc::now().day() as u8;

            let mut age = current_year.saturating_sub(birth_year as u16);

            // If birthday hasn't occurred this year yet, subtract 1
            if current_month < self.birth_month as u8
                || (current_month == self.birth_month as u8 && current_day < self.birth_day as u8)
            {
                age = age.saturating_sub(1);
            }

            age
        })
    }

    /// Calculate days until next birthday
    pub fn days_until_birthday(&self) -> i32 {
        use chrono::{Datelike, NaiveDate};

        let now = Utc::now().naive_utc().date();
        let current_year = now.year();

        // Try to create birthday date for current year
        let birthday_this_year =
            NaiveDate::from_ymd_opt(current_year, self.birth_month as u32, self.birth_day as u32);

        let target_date = match birthday_this_year {
            Some(date) => {
                if date >= now {
                    date // Birthday is later this year
                } else {
                    // Birthday already passed this year, use next year
                    NaiveDate::from_ymd_opt(
                        current_year + 1,
                        self.birth_month as u32,
                        self.birth_day as u32,
                    )
                    .unwrap_or(date)
                }
            }
            None => {
                // Invalid date (e.g., Feb 29 in non-leap year), use next valid occurrence
                NaiveDate::from_ymd_opt(
                    current_year + 1,
                    self.birth_month as u32,
                    self.birth_day as u32,
                )
                .unwrap_or(now)
            }
        };

        (target_date - now).num_days() as i32
    }

    /// Check if today is this person's birthday
    pub fn is_birthday_today(&self) -> bool {
        use chrono::Datelike;
        let now = Utc::now();
        now.month() as u8 == self.birth_month as u8 && now.day() as u8 == self.birth_day as u8
    }

    /// Create a BirthdayWithAge from this birthday
    pub fn with_age(self) -> BirthdayWithAge {
        let age = self.calculate_age();
        let days_until_birthday = self.days_until_birthday();

        BirthdayWithAge {
            birthday: self,
            age,
            days_until_birthday,
        }
    }
}

impl CreateBirthdayInput {
    /// Validates that the provided date is correct, including leap year logic.
    pub fn validate_date(&self) -> Result<(), String> {
        // Check month range
        if !(1..=12).contains(&self.birth_month) {
            return Err("Invalid month".to_string());
        }

        // Determine max days in month
        let max_day = match self.birth_month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.birth_day == 29 {
                    // Only allow Feb 29 if year is leap or year is not specified
                    if let Some(year) = self.birth_year {
                        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
                        if !is_leap {
                            return Err("February 29th is only valid in leap years".to_string());
                        }
                    }
                    29
                } else {
                    28.max(self.birth_day)
                }
            }
            _ => unreachable!(), // Already checked month range above
        };

        if self.birth_day == 0 || self.birth_day > max_day {
            return Err(format!(
                "Day {} is invalid for month {}",
                self.birth_day, self.birth_month
            ));
        }

        Ok(())
    }
}
