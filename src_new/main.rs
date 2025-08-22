mod config;
mod database;
mod errors;
mod handlers;
mod localization;
mod middleware;
mod services;
mod utils;

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use teloxide::{prelude::Requester, Bot};
use tokio::signal;
use tracing::{error, info};

use config::Settings;
use database::Database;
use errors::{BotError, BotResult};

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Skip database migrations
    #[arg(long)]
    skip_migrations: bool,

    /// Run only migrations and exit
    #[arg(long)]
    migrate_only: bool,
}

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Settings>,
    pub database: Database,
    pub bot: Bot,
    pub services: services::ServiceContainer,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    init_logging()?;

    info!("🎂 Starting Birthday Bot v2...");

    // Load configuration
    let config = Arc::new(Settings::new().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?);

    // Validate configuration
    config.validate().map_err(|e| {
        error!("Configuration validation failed: {}", e);
        e
    })?;

    info!("✅ Configuration loaded successfully");

    // Connect to database
    let database = Database::connect(&config.database).await.map_err(|e| {
        error!("Failed to connect to database: {}", e);
        e
    })?;

    info!("✅ Database connected successfully");

    // Run migrations
    if !args.skip_migrations {
        info!("🔄 Running database migrations...");
        database.migrate().await.map_err(|e| {
            error!("Database migration failed: {}", e);
            e
        })?;
        info!("✅ Database migrations completed");
    }

    // If migrate_only flag is set, exit after migrations
    if args.migrate_only {
        info!("✅ Migration completed, exiting as requested");
        return Ok(());
    }

    // Initialize bot
    let token = config.get_telegram_token().map_err(|e| {
        error!("Failed to get Telegram token: {}", e);
        e
    })?;

    let bot = Bot::new(token);

    // Test bot connection
    match bot.get_me().await {
        Ok(me) => {
            info!("✅ Bot connected successfully: @{}", me.username());
        }
        Err(e) => {
            error!("Failed to connect to Telegram: {}", e);
            return Err(e.into());
        }
    }

    // Initialize service container
    let services = services::ServiceContainer::new(config.clone(), database.clone());

    // Create application state
    let app_state = AppState {
        config: config.clone(),
        database: database.clone(),
        bot: bot.clone(),
        services,
    };

    // Start background tasks
    tokio::spawn(start_background_tasks(app_state.clone()));

    // Start bot
    info!("🔍 Webhook URL: '{}'", config.telegram.webhook_url);
    info!("🔍 Is webhook mode: {}", config.is_webhook_mode());

    if config.is_webhook_mode() {
        info!("🚀 Starting bot in webhook mode...");
        start_webhook_mode(app_state).await?;
    } else {
        info!("🚀 Starting bot in polling mode...");
        start_polling_mode(app_state).await?;
    }

    Ok(())
}

/// Initialize logging
fn init_logging() -> Result<()> {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("birthday_bot_v2=info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    Ok(())
}

/// Start bot in polling mode
async fn start_polling_mode(app_state: AppState) -> BotResult<()> {
    use teloxide::prelude::*;

    info!("Starting bot dispatcher...");

    let handler = handlers::create_handler();
    let bot = app_state.bot.clone();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![app_state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

/// Start bot in webhook mode
async fn start_webhook_mode(app_state: AppState) -> BotResult<()> {
    use teloxide::prelude::*;
    use teloxide::types::WebhookInfo;

    let webhook_url = app_state.config.telegram.webhook_url.clone();
    let bot = app_state.bot.clone();

    // Set webhook
    info!("Setting webhook to: {}", webhook_url);
    let webhook_url_parsed = webhook_url.parse().map_err(|e| {
        error!("Failed to parse webhook URL: {}", e);
        BotError::internal("Invalid webhook URL")
    })?;
    bot.set_webhook(webhook_url_parsed).await.map_err(|e| {
        error!("Failed to set webhook: {}", e);
        BotError::Telegram(e)
    })?;

    // Verify webhook is set
    let webhook_info: WebhookInfo = bot.get_webhook_info().await.map_err(|e| {
        error!("Failed to get webhook info: {}", e);
        BotError::Telegram(e)
    })?;

    info!("Webhook info: {:?}", webhook_info);

    // For now, use polling mode as webhook server setup is complex
    // TODO: Implement proper webhook server with axum or warp
    info!("Webhook set successfully, but using polling mode for now");
    start_polling_mode(app_state).await?;

    Ok(())
}

/// Start background tasks
async fn start_background_tasks(app_state: AppState) {
    info!("🔄 Starting background tasks...");

    let cleanup_task = tokio::spawn(cleanup_task(app_state.database.clone()));
    let notification_task = tokio::spawn(notification_task(app_state.clone()));

    // Wait for shutdown signal
    let shutdown = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };

    tokio::select! {
        _ = cleanup_task => {
            error!("Cleanup task terminated unexpectedly");
        }
        _ = notification_task => {
            error!("Notification task terminated unexpectedly");
        }
        _ = shutdown => {
            info!("📴 Shutdown signal received, stopping background tasks...");
        }
    }
}

/// Cleanup task - runs periodically to clean up expired data
async fn cleanup_task(database: Database) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600)); // 1 hour

    loop {
        interval.tick().await;

        match database.cleanup_expired().await {
            Ok(stats) => {
                if stats.total_cleaned() > 0 {
                    info!(
                        "🧹 Cleanup completed: {} records cleaned",
                        stats.total_cleaned()
                    );
                }
            }
            Err(e) => {
                error!("Cleanup task failed: {}", e);
            }
        }
    }
}

/// Notification task - sends birthday reminders
async fn notification_task(app_state: AppState) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // 1 minute

    loop {
        interval.tick().await;

        // Check for birthdays and send notifications
        let current_time = chrono::Utc::now();
        match app_state
            .services
            .notification_service()
            .should_send_notifications_now(current_time)
            .await
        {
            Ok(should_send) => {
                if should_send {
                    info!("📅 Checking for birthday notifications");
                    // TODO: Implement actual notification sending logic
                    // This would involve:
                    // 1. Getting birthdays for today
                    // 2. Getting notification settings for each chat
                    // 3. Sending messages to appropriate chats
                }
            }
            Err(e) => {
                error!("Failed to check notification status: {}", e);
            }
        }
    }
}
