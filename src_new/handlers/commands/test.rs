use teloxide::prelude::*;

use crate::localization::Messages;
use crate::AppState;

/// Handle /test command
pub(crate) async fn handle_test(bot: Bot, msg: Message, app_state: AppState) -> ResponseResult<()> {
    let chat_id = msg.chat.id.0;
    let user_id = msg.from().map(|u| u.id.0 as i64).unwrap_or(0);

    tracing::info!("Test command executed for chat_id: {}", chat_id);

    // Test database connection and queries
    let mut test_results = Vec::new();

    // Test 1: Count total birthdays
    match app_state
        .services
        .birthday_service()
        .get_chat_statistics(chat_id)
        .await
    {
        Ok(stats) => {
            let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::stats_success(stats.total_birthdays as i32)
            })
            .await;
            test_results.push(text);
        }
        Err(e) => {
            let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::test_stats_error(&e.to_string())
            })
            .await;
            test_results.push(text);
        }
    }

    // Test 2: Get birthdays for this chat
    let filter = crate::database::models::BirthdayFilter {
        chat_id: Some(chat_id),
        limit: Some(5),
        ..Default::default()
    };

    match app_state
        .services
        .birthday_service()
        .get_birthdays(filter)
        .await
    {
        Ok(birthdays) => {
            let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::test_found_birthdays(birthdays.len())
            })
            .await;
            test_results.push(text);

            if !birthdays.is_empty() {
                let sample = &birthdays[0];
                let sample_text =
                    crate::handlers::get_user_localized_message(&app_state, user_id, || {
                        Messages::test_sample_birthday(
                            &sample.name,
                            sample.birth_day as i32,
                            sample.birth_month as i32,
                        )
                    })
                    .await;
                test_results.push(sample_text);
            }
        }
        Err(e) => {
            let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::test_birthday_error_general(&e.to_string())
            })
            .await;
            test_results.push(text);
        }
    }

    // Test 3: Add a test birthday
    let test_input = crate::database::models::CreateBirthdayInput {
        name: crate::handlers::get_user_localized_message(&app_state, user_id, || {
            Messages::test_user_name()
        })
        .await,
        birth_month: 1,
        birth_day: 15,
        birth_year: Some(1990),
        username: Some("test_user".to_string()),
        notes: Some(
            crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::test_user_notes()
            })
            .await,
        ),
    };

    match app_state
        .services
        .birthday_service()
        .add_birthday(chat_id, test_input, chat_id)
        .await
    {
        Ok(_) => {
            let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                Messages::test_birthday_added()
            })
            .await;
            test_results.push(text);
        }
        Err(e) => {
            if e.to_string().contains("already exists") {
                let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                    Messages::test_birthday_exists()
                })
                .await;
                test_results.push(text);
            } else {
                let text = crate::handlers::get_user_localized_message(&app_state, user_id, || {
                    Messages::test_birthday_error(&e.to_string())
                })
                .await;
                test_results.push(text);
            }
        }
    }

    let title = crate::handlers::get_user_localized_message(&app_state, user_id, || {
        Messages::test_results_title()
    })
    .await;

    let test_text = format!("{}\n\n{}", title, test_results.join("\n"));

    bot.send_message(msg.chat.id, test_text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}
