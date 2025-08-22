use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::{config::DatabaseConfig, errors::BotResult};

const HEALTH_CHECK_QUERY: &str = r#"
    SELECT 1
"#;

const VACUUM_ANALYZE_QUERY: &str = r#"
    VACUUM ANALYZE
"#;

const GET_DATABASE_STATS_QUERY: &str = r#"
    SELECT 
        (SELECT COUNT(*) FROM chats WHERE is_active = true) as active_chats,
        (SELECT COUNT(*) FROM birthdays) as total_birthdays,
        (SELECT COUNT(*) FROM users) as total_users,
        (SELECT COUNT(*) FROM conversation_states WHERE expires_at > NOW()) as active_conversations
"#;

const CLEANUP_EXPIRED_CONVERSATION_STATES_QUERY: &str = r#"
    DELETE FROM conversation_states WHERE expires_at < NOW()
"#;

const CLEANUP_OLD_RATE_LIMITS_QUERY: &str = r#"
    DELETE FROM rate_limits WHERE window_start < NOW() - INTERVAL '24 hours' AND NOT is_blocked
"#;

const CLEANUP_OLD_AUDIT_LOGS_QUERY: &str = r#"
    DELETE FROM audit_log WHERE created_at < NOW() - INTERVAL '90 days'
"#;

const CLEANUP_OLD_METRICS_QUERY: &str = r#"
    DELETE FROM metrics WHERE timestamp < NOW() - INTERVAL '30 days'
"#;

/// Database connection manager
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn connect(config: &DatabaseConfig) -> BotResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.connection_timeout))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    /// Get the underlying connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn migrate(&self) -> BotResult<()> {
        sqlx::migrate!("src_new/database/migrations")
            .run(&self.pool)
            .await
            .map_err(|e| crate::errors::BotError::Database(sqlx::Error::Migrate(Box::new(e))))?;
        Ok(())
    }

    /// Check database health
    pub async fn health_check(&self) -> BotResult<()> {
        sqlx::query(HEALTH_CHECK_QUERY).execute(&self.pool).await?;
        Ok(())
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> BotResult<DatabaseStats> {
        let row = sqlx::query(GET_DATABASE_STATS_QUERY)
            .fetch_one(&self.pool)
            .await?;

        let stats = DatabaseStats {
            active_chats: row.get("active_chats"),
            total_birthdays: row.get("total_birthdays"),
            total_users: row.get("total_users"),
            active_conversations: row.get("active_conversations"),
        };

        Ok(stats)
    }

    /// Clean up expired data
    pub async fn cleanup_expired(&self) -> BotResult<CleanupStats> {
        let mut tx = self.pool.begin().await?;

        // Clean up expired conversation states
        let expired_conversations = sqlx::query(CLEANUP_EXPIRED_CONVERSATION_STATES_QUERY)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        // Clean up old rate limit records (older than 24 hours and not blocked)
        let expired_rate_limits = sqlx::query(CLEANUP_OLD_RATE_LIMITS_QUERY)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        // Clean up old audit log entries (older than 90 days)
        let expired_audit_logs = sqlx::query(CLEANUP_OLD_AUDIT_LOGS_QUERY)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        // Clean up old metrics (older than 30 days)
        let expired_metrics = sqlx::query(CLEANUP_OLD_METRICS_QUERY)
            .execute(&mut *tx)
            .await?
            .rows_affected();

        tx.commit().await?;

        Ok(CleanupStats {
            expired_conversations,
            expired_rate_limits,
            expired_audit_logs,
            expired_metrics,
        })
    }

    /// Vacuum and analyze database for maintenance
    pub async fn maintenance(&self) -> BotResult<()> {
        // Note: VACUUM and ANALYZE cannot be run in a transaction
        sqlx::query(VACUUM_ANALYZE_QUERY)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

/// Database statistics
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub active_chats: Option<i64>,
    pub total_birthdays: Option<i64>,
    pub total_users: Option<i64>,
    pub active_conversations: Option<i64>,
}

/// Cleanup operation statistics
#[derive(Debug, Clone)]
pub struct CleanupStats {
    pub expired_conversations: u64,
    pub expired_rate_limits: u64,
    pub expired_audit_logs: u64,
    pub expired_metrics: u64,
}

impl CleanupStats {
    /// Get total number of cleaned up records
    pub fn total_cleaned(&self) -> u64 {
        self.expired_conversations
            + self.expired_rate_limits
            + self.expired_audit_logs
            + self.expired_metrics
    }
}
