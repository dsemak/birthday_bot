use crate::{
    database::{
        models::{Birthday, BirthdayFilter, CreateBirthdayInput},
        repository::{BirthdayRepository, BirthdayStats},
    },
    errors::BotResult,
};

/// Service for birthday business logic
/// Follows Single Responsibility Principle
#[derive(Debug, Clone)]
pub struct BirthdayService {
    repository: BirthdayRepository,
}

impl BirthdayService {
    /// Create new birthday service
    pub fn new(repository: BirthdayRepository) -> Self {
        Self { repository }
    }

    /// Add a new birthday
    /// Handles validation and business rules
    pub async fn add_birthday(
        &self,
        chat_id: i64,
        input: CreateBirthdayInput,
        created_by: i64,
    ) -> BotResult<Birthday> {
        // Business rule: Check if we're approaching the limit
        let current_count = self.repository.count_by_chat(chat_id).await?;
        const MAX_BIRTHDAYS_PER_CHAT: i64 = 1000;

        if current_count >= MAX_BIRTHDAYS_PER_CHAT {
            return Err(crate::errors::BotError::resource_limit(format!(
                "Maximum {} birthdays per chat exceeded",
                MAX_BIRTHDAYS_PER_CHAT
            )));
        }

        // Delegate to repository
        self.repository.create(chat_id, input, created_by).await
    }

    /// Get birthdays with optional filtering
    pub async fn get_birthdays(&self, filter: BirthdayFilter) -> BotResult<Vec<Birthday>> {
        tracing::debug!(
            "BirthdayService::get_birthdays called with filter: {:?}",
            filter
        );
        let result = self.repository.get_by_filter(filter).await;
        match &result {
            Ok(birthdays) => {
                tracing::debug!(
                    "BirthdayService::get_birthdays returned {} birthdays",
                    birthdays.len()
                );
            }
            Err(e) => {
                tracing::error!("BirthdayService::get_birthdays failed: {}", e);
            }
        }
        result
    }

    /// Get birthday statistics for a chat
    pub async fn get_chat_statistics(&self, chat_id: i64) -> BotResult<BirthdayStats> {
        self.repository.get_chat_stats(chat_id).await
    }

    /// Search birthdays by name
    pub async fn search_birthdays(
        &self,
        chat_id: i64,
        query: &str,
        limit: Option<i64>,
    ) -> BotResult<Vec<Birthday>> {
        let limit = limit.unwrap_or(10);
        self.repository.search_by_name(chat_id, query, limit).await
    }

    /// Get upcoming birthdays
    pub async fn get_upcoming_birthdays(
        &self,
        chat_id: i64,
        days_ahead: u32,
    ) -> BotResult<Vec<Birthday>> {
        // Business logic for calculating upcoming birthdays
        let mut filter = BirthdayFilter::default();
        filter.chat_id = Some(chat_id);

        let all_birthdays = self.repository.get_by_filter(filter).await?;

        // Filter for upcoming birthdays (simplified implementation)
        let upcoming: Vec<Birthday> = all_birthdays
            .into_iter()
            .filter(|birthday| {
                let days_until = birthday.days_until_birthday();
                days_until >= 0 && days_until <= days_ahead as i32
            })
            .collect();

        Ok(upcoming)
    }

    /// Check if today has any birthdays in this chat
    pub async fn get_todays_birthdays(&self, chat_id: i64) -> BotResult<Vec<Birthday>> {
        use chrono::{Datelike, Utc};

        let now = Utc::now();
        let month = now.month() as u8;
        let day = now.day() as u8;

        let mut filter = BirthdayFilter::default();
        filter.chat_id = Some(chat_id);
        filter.birth_month = Some(month);
        filter.birth_day = Some(day);

        self.repository.get_by_filter(filter).await
    }
}

/// Factory for creating BirthdayService instances
/// Follows Dependency Inversion Principle
pub struct BirthdayServiceFactory;

impl BirthdayServiceFactory {
    pub fn create(repository: BirthdayRepository) -> BirthdayService {
        BirthdayService::new(repository)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests would go here
    // Testing business logic separately from infrastructure
}
