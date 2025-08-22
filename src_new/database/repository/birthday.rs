use sqlx::{PgPool, Row};

use crate::{
    database::models::{Birthday, BirthdayFilter, CreateBirthdayInput},
    errors::{BotError, BotResult},
};

const CREATE_BIRTHDAY_QUERY: &str = r#"
    INSERT INTO birthdays (chat_id, name, birth_month, birth_day, birth_year, username, notes, created_by)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING id, chat_id, name, birth_month, birth_day, birth_year, username, notes, created_by, created_at, updated_at
"#;

const GET_BIRTHDAYS_QUERY: &str = r#"
    SELECT id, chat_id, name, birth_month, birth_day, birth_year, username, notes, created_by, created_at, updated_at FROM birthdays WHERE chat_id = $1 ORDER BY birth_month, birth_day, name
"#;

const GET_BIRTHDAYS_QUERY_ALL: &str = r#"
    SELECT id, chat_id, name, birth_month, birth_day, birth_year, username, notes, created_by, created_at, updated_at FROM birthdays ORDER BY birth_month, birth_day, name
"#;

const GET_BIRTHDAYS_COUNT_QUERY: &str = r#"
    SELECT COUNT(*) FROM birthdays WHERE chat_id = $1
"#;

const GET_BIRTHDAYS_STATS_QUERY: &str = r#"
    SELECT 
        COUNT(*) as total_birthdays,
        COUNT(CASE WHEN birth_month = EXTRACT(MONTH FROM CURRENT_DATE) AND birth_day = EXTRACT(DAY FROM CURRENT_DATE) THEN 1 END) as birthdays_today,
        COUNT(CASE WHEN birth_year IS NOT NULL THEN 1 END) as birthdays_with_year
    FROM birthdays
    WHERE chat_id = $1
"#;

const GET_BIRTHDAYS_SEARCH_QUERY: &str = r#"
    SELECT id, chat_id, name, birth_month, birth_day, birth_year, username, notes, created_by, created_at, updated_at
    FROM birthdays 
    WHERE chat_id = $1 AND name ILIKE $2
    ORDER BY name
    LIMIT $3
"#;

/// Birthday statistics for a chat
#[derive(Debug, Clone)]
pub struct BirthdayStats {
    pub total_birthdays: i64,
    pub birthdays_today: i64,
    pub birthdays_with_year: i64,
}

/// Simplified repository for managing birthday data
#[derive(Debug, Clone)]
pub struct BirthdayRepository {
    pool: PgPool,
}

impl BirthdayRepository {
    /// Create new repository instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new birthday
    pub async fn create(
        &self,
        chat_id: i64,
        input: CreateBirthdayInput,
        created_by: i64,
    ) -> BotResult<Birthday> {
        // Validate input
        validator::Validate::validate(&input)?;
        input.validate_date().map_err(BotError::validation)?;

        let row = sqlx::query(CREATE_BIRTHDAY_QUERY)
            .bind(chat_id)
            .bind(&input.name)
            .bind(input.birth_month as i16)
            .bind(input.birth_day as i16)
            .bind(input.birth_year.map(|y| y as i16))
            .bind(&input.username)
            .bind(&input.notes)
            .bind(created_by)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.constraint().is_some() {
                        return BotError::validation(
                            "Birthday with this name and date already exists in this chat",
                        );
                    }
                }
                BotError::Database(e)
            })?;

        let birthday = Birthday {
            id: row.get("id"),
            chat_id: row.get("chat_id"),
            name: row.get("name"),
            birth_month: row.get("birth_month"),
            birth_day: row.get("birth_day"),
            birth_year: row.get("birth_year"),
            username: row.get("username"),
            notes: row.get("notes"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(birthday)
    }

    /// Get birthdays by filter
    pub async fn get_by_filter(&self, filter: BirthdayFilter) -> BotResult<Vec<Birthday>> {
        tracing::debug!(
            "BirthdayRepository::get_by_filter called with filter: {:?}",
            filter
        );

        let rows = if let Some(chat_id) = filter.chat_id {
            tracing::debug!("Executing query for chat_id: {}", chat_id);
            sqlx::query(GET_BIRTHDAYS_QUERY)
                .bind(chat_id)
                .fetch_all(&self.pool)
                .await?
        } else {
            tracing::debug!("Executing query for all chats");
            sqlx::query(GET_BIRTHDAYS_QUERY_ALL)
                .fetch_all(&self.pool)
                .await?
        };

        tracing::debug!("Query returned {} rows", rows.len());

        let birthdays: Vec<Birthday> = rows
            .into_iter()
            .map(|row| Birthday {
                id: row.get("id"),
                chat_id: row.get("chat_id"),
                name: row.get("name"),
                birth_month: row.get("birth_month"),
                birth_day: row.get("birth_day"),
                birth_year: row.get("birth_year"),
                username: row.get("username"),
                notes: row.get("notes"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        tracing::debug!("Returning {} birthdays", birthdays.len());
        Ok(birthdays)
    }

    /// Count birthdays in chat
    pub async fn count_by_chat(&self, chat_id: i64) -> BotResult<i64> {
        let count: i64 = sqlx::query_scalar(GET_BIRTHDAYS_COUNT_QUERY)
            .bind(chat_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    /// Get birthday statistics for chat
    pub async fn get_chat_stats(&self, chat_id: i64) -> BotResult<BirthdayStats> {
        let row = sqlx::query(GET_BIRTHDAYS_STATS_QUERY)
            .bind(chat_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(BirthdayStats {
            total_birthdays: row.get("total_birthdays"),
            birthdays_today: row.get("birthdays_today"),
            birthdays_with_year: row.get("birthdays_with_year"),
        })
    }

    /// Get birthdays for a specific date
    pub async fn get_birthdays_for_date(&self, month: u8, day: u8) -> BotResult<Vec<Birthday>> {
        let filter = BirthdayFilter {
            birth_month: Some(month),
            birth_day: Some(day),
            ..BirthdayFilter::default()
        };
        self.get_by_filter(filter).await
    }

    /// Search birthdays by name
    pub async fn search_by_name(
        &self,
        chat_id: i64,
        query: &str,
        limit: i64,
    ) -> BotResult<Vec<Birthday>> {
        let rows = sqlx::query(GET_BIRTHDAYS_SEARCH_QUERY)
            .bind(chat_id)
            .bind(format!("%{query}%"))
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut birthdays = Vec::new();
        for row in rows {
            use sqlx::Row;
            let birthday = Birthday {
                id: row.get("id"),
                chat_id: row.get("chat_id"),
                name: row.get("name"),
                birth_month: row.get("birth_month"),
                birth_day: row.get("birth_day"),
                birth_year: row.get("birth_year"),
                username: row.get("username"),
                notes: row.get("notes"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            birthdays.push(birthday);
        }
        Ok(birthdays)
    }
}
