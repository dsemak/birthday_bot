use crate::database::models::BirthdayPreview;

/// Format single birthday preview for confirmation
pub fn format_birthday_preview(preview: &BirthdayPreview) -> String {
    let mut lines = vec![
        "📋 **Предварительный просмотр дня рождения:**".to_string(),
        "".to_string(),
    ];

    lines.push(format!("👤 **Имя:** {}", preview.name));

    let date_str =
        crate::utils::format_date(preview.birth_day, preview.birth_month, preview.birth_year);
    lines.push(format!("📅 **Дата:** {}", date_str));

    if let Some(ref username) = preview.username {
        lines.push(format!("🔗 **Username:** @{}", username));
    }

    if let Some(ref notes) = preview.notes {
        lines.push(format!("📝 **Заметки:** {}", notes));
    }

    lines.push("".to_string());
    lines.push("Подтвердите добавление (да/нет):".to_string());

    lines.join("\n")
}

/// Format batch preview for confirmation
pub fn format_batch_preview(previews: &[BirthdayPreview]) -> String {
    let mut lines = vec![
        format!(
            "📋 **Предварительный просмотр ({} записей):**",
            previews.len()
        ),
        "".to_string(),
    ];

    lines.push(format!("✅ Записей к добавлению: {}", previews.len()));

    // Show first few previews
    let show_count = std::cmp::min(5, previews.len());
    for (i, preview) in previews.iter().take(show_count).enumerate() {
        lines.push("".to_string());
        lines.push(format!("{}. {}", i + 1, preview.name));

        let date_str =
            crate::utils::format_date(preview.birth_day, preview.birth_month, preview.birth_year);
        lines.push(format!("   📅 {}", date_str));

        if let Some(ref username) = preview.username {
            lines.push(format!("   🔗 @{}", username));
        }
    }

    if previews.len() > show_count {
        lines.push(format!("... и еще {} записей", previews.len() - show_count));
    }

    lines.push("".to_string());
    lines.push("Подтвердите добавление (да/нет):".to_string());

    lines.join("\n")
}

/// Format batch operation result
pub fn format_batch_result(saved_count: usize, errors: Vec<String>) -> String {
    let mut lines = vec![
        "📊 **Результат батчевого добавления:**".to_string(),
        "".to_string(),
        format!("✅ Успешно добавлено: {}", saved_count),
    ];

    if !errors.is_empty() {
        lines.push(format!("❌ Ошибок: {}", errors.len()));
        lines.push("".to_string());
        lines.push("Детали ошибок:".to_string());

        let max_errors_to_show = 5;
        for error in errors.iter().take(max_errors_to_show) {
            lines.push(format!("• {}", error));
        }

        if errors.len() > max_errors_to_show {
            lines.push(format!(
                "... и еще {} ошибок",
                errors.len() - max_errors_to_show
            ));
        }
    }

    lines.join("\n")
}
