use sqlx::{PgPool, Row};

use crate::{
    database::models::{
        ChatMember, CreateChatMemberInput, CreateUserInput, User, UserRole, UserWithRole,
    },
    errors::BotResult,
};

const UPSERT_USER_QUERY: &str = r#"
    INSERT INTO users (user_id, username, first_name, last_name, language_code, is_bot)
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT (user_id) DO UPDATE SET
        username = EXCLUDED.username,
        first_name = EXCLUDED.first_name,
        last_name = EXCLUDED.last_name,
        language_code = EXCLUDED.language_code,
        updated_at = NOW()
    RETURNING user_id, username, first_name, last_name, language_code, is_bot, created_at, updated_at
"#;

const GET_USER_QUERY: &str = r#"
    SELECT user_id, username, first_name, last_name, language_code, is_bot, created_at, updated_at FROM users WHERE user_id = $1
"#;

const UPSERT_CHAT_MEMBER_QUERY: &str = r#"
    INSERT INTO chat_members (chat_id, user_id, role)
    VALUES ($1, $2, $3)
    ON CONFLICT (chat_id, user_id) DO UPDATE SET
        role = EXCLUDED.role
    RETURNING chat_id, user_id, role, joined_at
"#;

const GET_USER_ROLE_QUERY: &str = r#"
    SELECT role FROM chat_members WHERE chat_id = $1 AND user_id = $2
"#;

const UPDATE_USER_ROLE_QUERY: &str = r#"
    UPDATE chat_members SET role = $1 WHERE chat_id = $2 AND user_id = $3
"#;

const GET_USER_WITH_ROLE_QUERY: &str = r#"
    SELECT u.user_id, u.username, u.first_name, u.last_name, u.language_code, u.is_bot, u.created_at, u.updated_at,
           cm.role, cm.joined_at
    FROM users u
    JOIN chat_members cm ON u.user_id = cm.user_id
    WHERE cm.chat_id = $1 AND cm.user_id = $2
"#;

/// Simplified repository for managing user data
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Create new repository instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create or update user
    pub async fn upsert(&self, input: CreateUserInput) -> BotResult<User> {
        let row = sqlx::query(UPSERT_USER_QUERY)
            .bind(input.user_id)
            .bind(&input.username)
            .bind(&input.first_name)
            .bind(&input.last_name)
            .bind(&input.language_code)
            .bind(input.is_bot)
            .fetch_one(&self.pool)
            .await?;

        let user = User {
            user_id: row.get("user_id"),
            username: row.get("username"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            language_code: row.get("language_code"),
            is_bot: row.get("is_bot"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };

        Ok(user)
    }

    /// Get user by ID
    pub async fn get_by_id(&self, user_id: i64) -> BotResult<Option<User>> {
        let row = sqlx::query(GET_USER_QUERY)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let user = User {
                user_id: row.get("user_id"),
                username: row.get("username"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                language_code: row.get("language_code"),
                is_bot: row.get("is_bot"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    /// Add user to chat with role
    pub async fn add_to_chat(&self, input: CreateChatMemberInput) -> BotResult<ChatMember> {
        let row = sqlx::query(UPSERT_CHAT_MEMBER_QUERY)
            .bind(input.chat_id)
            .bind(input.user_id)
            .bind(role_to_string(&input.role))
            .fetch_one(&self.pool)
            .await?;

        let member = ChatMember {
            chat_id: row.get("chat_id"),
            user_id: row.get("user_id"),
            role: string_to_role(row.get("role")),
            joined_at: row.get("joined_at"),
        };

        Ok(member)
    }

    /// Get user role in chat
    pub async fn get_user_role(&self, chat_id: i64, user_id: i64) -> BotResult<Option<UserRole>> {
        let role_str: Option<String> = sqlx::query_scalar(GET_USER_ROLE_QUERY)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(role_str.map(|s| string_to_role(&s)))
    }

    /// Update user role in chat
    pub async fn update_role(
        &self,
        chat_id: i64,
        user_id: i64,
        new_role: UserRole,
    ) -> BotResult<bool> {
        let result = sqlx::query(UPDATE_USER_ROLE_QUERY)
            .bind(role_to_string(&new_role))
            .bind(chat_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Get user with role in chat
    pub async fn get_user_with_role(
        &self,
        chat_id: i64,
        user_id: i64,
    ) -> BotResult<Option<UserWithRole>> {
        let row = sqlx::query(GET_USER_WITH_ROLE_QUERY)
            .bind(chat_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let user_with_role = UserWithRole {
                user: User {
                    user_id: row.get("user_id"),
                    username: row.get("username"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    language_code: row.get("language_code"),
                    is_bot: row.get("is_bot"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                },
                role: string_to_role(row.get("role")),
                joined_at: row.get("joined_at"),
            };
            Ok(Some(user_with_role))
        } else {
            Ok(None)
        }
    }
}

fn role_to_string(role: &UserRole) -> &'static str {
    match role {
        UserRole::Maintainer => "maintainer",
        UserRole::Admin => "admin",
        UserRole::Member => "member",
        UserRole::Restricted => "restricted",
    }
}

fn string_to_role(s: &str) -> UserRole {
    match s {
        "maintainer" => UserRole::Maintainer,
        "admin" => UserRole::Admin,
        "member" => UserRole::Member,
        "restricted" => UserRole::Restricted,
        _ => UserRole::Member,
    }
}
