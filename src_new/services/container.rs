use std::sync::Arc;

use crate::{
    config::Settings,
    database::{repository::*, Database},
    services::*,
};

/// Service container for dependency injection
/// Follows Dependency Inversion Principle
#[derive(Debug, Clone)]
pub struct ServiceContainer {
    // Core dependencies
    pub config: Arc<Settings>,
    pub database: Database,

    // Repositories
    pub birthday_repo: BirthdayRepository,
    pub chat_repo: ChatRepository,
    pub user_repo: UserRepository,
    pub notification_repo: NotificationRepository,
    pub conversation_repo: ConversationRepository,
    pub rate_limit_repo: RateLimitRepository,

    // Services
    pub birthday_service: BirthdayService,
    pub user_service: UserService,
    pub notification_service: NotificationService,
    pub conversation_service: ConversationService,
    pub localization_service: LocalizationService,
}

impl ServiceContainer {
    /// Create new service container with all dependencies
    pub fn new(config: Arc<Settings>, database: Database) -> Self {
        let pool = database.pool().clone();

        // Initialize repositories
        let birthday_repo = BirthdayRepository::new(pool.clone());
        let chat_repo = ChatRepository::new(pool.clone());
        let user_repo = UserRepository::new(pool.clone());
        let notification_repo = NotificationRepository::new(pool.clone());
        let conversation_repo = ConversationRepository::new(pool.clone());
        let rate_limit_repo = RateLimitRepository::new(pool);

        // Initialize services with injected dependencies
        let birthday_service = BirthdayServiceFactory::create(birthday_repo.clone());
        let user_service = UserServiceFactory::create(user_repo.clone(), config.clone());
        let notification_service = NotificationServiceFactory::create(
            notification_repo.clone(),
            chat_repo.clone(),
            birthday_repo.clone(),
        );
        let conversation_service = ConversationService::new(conversation_repo.clone());
        let localization_service = LocalizationService::new(Arc::new(user_service.clone()));

        Self {
            config,
            database,
            birthday_repo,
            chat_repo,
            user_repo,
            notification_repo,
            conversation_repo,
            rate_limit_repo,
            birthday_service,
            user_service,
            notification_service,
            conversation_service,
            localization_service,
        }
    }

    /// Get birthday service
    pub fn birthday_service(&self) -> &BirthdayService {
        &self.birthday_service
    }

    /// Get user service
    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }

    /// Get notification service
    pub fn notification_service(&self) -> &NotificationService {
        &self.notification_service
    }

    /// Get conversation service
    pub fn conversation_service(&self) -> &ConversationService {
        &self.conversation_service
    }

    /// Get localization service
    pub fn localization_service(&self) -> &LocalizationService {
        &self.localization_service
    }

    /// Get birthday repository
    pub fn birthday_repository(&self) -> &BirthdayRepository {
        &self.birthday_repo
    }

    /// Get user repository
    pub fn user_repository(&self) -> &UserRepository {
        &self.user_repo
    }

    /// Get chat repository
    pub fn chat_repository(&self) -> &ChatRepository {
        &self.chat_repo
    }
}

/// Builder for service container
/// Makes it easy to mock dependencies for testing
#[allow(dead_code)]
pub struct ServiceContainerBuilder {
    config: Option<Arc<Settings>>,
    database: Option<Database>,
}

impl ServiceContainerBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            config: None,
            database: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_config(mut self, config: Arc<Settings>) -> Self {
        self.config = Some(config);
        self
    }

    #[allow(dead_code)]
    pub fn with_database(mut self, database: Database) -> Self {
        self.database = Some(database);
        self
    }

    #[allow(dead_code)]
    pub fn build(self) -> Result<ServiceContainer, String> {
        let config = self.config.ok_or("Config is required")?;
        let database = self.database.ok_or("Database is required")?;

        Ok(ServiceContainer::new(config, database))
    }
}

impl Default for ServiceContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
