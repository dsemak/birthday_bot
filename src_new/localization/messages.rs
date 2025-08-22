use super::types::{Language, LocalizedText};

/// Centralized messages for the birthday bot
#[derive(Debug, Clone)]
pub struct Messages;

impl Messages {
    // Main menu messages
    pub fn main_menu_title() -> LocalizedText {
        LocalizedText::new(
            "📋 *Главное меню*\n\nВыберите действие:",
            "📋 *Main Menu*\n\nChoose an action:",
        )
    }

    // Add batch messages
    pub fn add_batch_upload_file() -> LocalizedText {
        LocalizedText::new(
            "📄 *Загрузка файла*\n\nОтправьте CSV файл со следующими колонками:\n\n`Имя,День,Месяц,Год,Username,Заметки`\n\nПример содержимого:\n```\nИван,15,3,1990,ivan,друг\nМария,22,12,,maria,\nАлексей,5,8,1985,alex,коллега\n```\n\n_Год, Username и Заметки необязательны_\\.\n\n🔙 Нажмите кнопку \\\"Назад\\\" для возврата\\.",
            "📄 *File Upload*\n\nSend a CSV file with the following columns:\n\n`Name,Day,Month,Year,Username,Notes`\n\nExample content:\n```\nJohn,15,3,1990,john,friend\nMary,22,12,,mary,\nAlex,5,8,1985,alex,colleague\n```\n\n_Year, Username and Notes are optional_\\.\n\n🔙 Click the \\\"Back\\\" button to return\\."
        )
    }

    // Button texts
    pub fn back_button() -> LocalizedText {
        LocalizedText::new("🔙 Назад", "🔙 Back")
    }

    pub fn add_single_button() -> LocalizedText {
        LocalizedText::new("➕ Добавить день рождения", "➕ Add Birthday")
    }

    pub fn add_batch_button() -> LocalizedText {
        LocalizedText::new("📄 Добавить из файла", "📄 Add from File")
    }

    pub fn list_button() -> LocalizedText {
        LocalizedText::new("📋 Список дней рождения", "📋 Birthday List")
    }

    pub fn search_button() -> LocalizedText {
        LocalizedText::new("🔍 Поиск", "🔍 Search")
    }

    pub fn export_button() -> LocalizedText {
        LocalizedText::new("📤 Экспорт", "📤 Export")
    }

    pub fn stats_button() -> LocalizedText {
        LocalizedText::new("📊 Статистика", "📊 Statistics")
    }

    pub fn settings_button() -> LocalizedText {
        LocalizedText::new("⚙️ Настройки", "⚙️ Settings")
    }

    // Error messages
    pub fn no_permission_private() -> LocalizedText {
        LocalizedText::new(
            "❌ У вас нет прав для использования этого бота.",
            "❌ You don't have permission to use this bot.",
        )
    }

    pub fn no_permission_group() -> LocalizedText {
        LocalizedText::new(
            "❌ Только администраторы могут использовать бота в группах и каналах.",
            "❌ Only administrators can use the bot in groups and channels.",
        )
    }

    pub fn no_permission_stats() -> LocalizedText {
        LocalizedText::new(
            "❌ Только администраторы могут просматривать статистику в группах и каналах.",
            "❌ Only administrators can view statistics in groups and channels.",
        )
    }

    pub fn no_permission_cancel() -> LocalizedText {
        LocalizedText::new(
            "❌ Только администраторы могут отменять операции в группах и каналах.",
            "❌ Only administrators can cancel operations in groups and channels.",
        )
    }

    // Help messages
    pub fn help_title() -> LocalizedText {
        LocalizedText::new(
            "🎂 *Birthday Bot v2*\n\nЭтот бот поможет вам не забывать о днях рождения!",
            "🎂 *Birthday Bot v2*\n\nThis bot will help you remember birthdays!",
        )
    }

    // Start messages
    pub fn start_welcome() -> LocalizedText {
        LocalizedText::new(
            "🎉 Добро пожаловать в Birthday Bot v2!\n\nПривет, {name}! Я помогу вам не забывать о днях рождения.\n\nИспользуйте /menu для открытия главного меню или /help для получения справки.",
            "🎉 Welcome to Birthday Bot v2!\n\nHello, {name}! I'll help you remember birthdays.\n\nUse /menu to open the main menu or /help for help."
        )
    }

    // Settings messages
    pub fn settings_title() -> LocalizedText {
        LocalizedText::new(
            "⚙️ *Настройки*\n\nВыберите настройку для изменения:",
            "⚙️ *Settings*\n\nChoose a setting to modify:",
        )
    }

    pub fn language_setting() -> LocalizedText {
        LocalizedText::new("🌐 Язык", "🌐 Language")
    }

    pub fn language_current(language: Language) -> LocalizedText {
        match language {
            Language::Russian => LocalizedText::new("🌐 Язык: Русский", "🌐 Language: Russian"),
            Language::English => LocalizedText::new("🌐 Язык: English", "🌐 Language: English"),
        }
    }

    pub fn language_changed(language: Language) -> LocalizedText {
        match language {
            Language::Russian => LocalizedText::new(
                "✅ Язык изменен на русский!",
                "✅ Language changed to Russian!",
            ),
            Language::English => LocalizedText::new(
                "✅ Language changed to English!",
                "✅ Language changed to English!",
            ),
        }
    }

    // List messages
    pub fn birthday_list_title() -> LocalizedText {
        LocalizedText::new(
            "📋 *Список дней рождения*\n\nВыберите период:",
            "📋 *Birthday List*\n\nChoose a period:",
        )
    }

    pub fn upcoming_birthdays() -> LocalizedText {
        LocalizedText::new("🎉 Ближайшие дни рождения", "🎉 Upcoming Birthdays")
    }

    pub fn all_birthdays() -> LocalizedText {
        LocalizedText::new("📅 Все дни рождения", "📅 All Birthdays")
    }

    pub fn this_month_birthdays() -> LocalizedText {
        LocalizedText::new("📅 Дни рождения в этом месяце", "📅 Birthdays This Month")
    }

    pub fn next_month_birthdays() -> LocalizedText {
        LocalizedText::new(
            "📅 Дни рождения в следующем месяце",
            "📅 Birthdays Next Month",
        )
    }

    // Search messages
    pub fn search_title() -> LocalizedText {
        LocalizedText::new(
            "🔍 *Поиск дней рождения*\n\nВведите имя для поиска:",
            "🔍 *Search Birthdays*\n\nEnter a name to search:",
        )
    }

    // Export messages
    pub fn export_title() -> LocalizedText {
        LocalizedText::new(
            "📤 *Экспорт данных*\n\nВыберите формат экспорта:",
            "📤 *Export Data*\n\nChoose export format:",
        )
    }

    pub fn export_csv() -> LocalizedText {
        LocalizedText::new("📄 CSV файл", "📄 CSV File")
    }

    pub fn export_json() -> LocalizedText {
        LocalizedText::new("📄 JSON файл", "📄 JSON File")
    }

    // Stats messages
    pub fn stats_title() -> LocalizedText {
        LocalizedText::new(
            "📊 *Статистика*\n\nВыберите тип статистики:",
            "📊 *Statistics*\n\nChoose statistics type:",
        )
    }

    pub fn general_stats() -> LocalizedText {
        LocalizedText::new("📈 Общая статистика", "📈 General Statistics")
    }

    pub fn monthly_stats() -> LocalizedText {
        LocalizedText::new("📅 Статистика по месяцам", "📅 Monthly Statistics")
    }

    // Add single birthday messages
    pub fn add_single_title() -> LocalizedText {
        LocalizedText::new(
            "➕ *Добавить день рождения*\n\nВведите имя человека:",
            "➕ *Add Birthday*\n\nEnter the person's name:",
        )
    }

    pub fn add_single_enter_day() -> LocalizedText {
        LocalizedText::new(
            "Введите день рождения \\(1\\-31\\):",
            "Enter the birthday day \\(1\\-31\\):",
        )
    }

    pub fn add_single_enter_month() -> LocalizedText {
        LocalizedText::new(
            "Введите месяц рождения \\(1\\-12\\):",
            "Enter the birthday month \\(1\\-12\\):",
        )
    }

    pub fn add_single_enter_year() -> LocalizedText {
        LocalizedText::new(
            "Введите год рождения \\(необязательно\\):",
            "Enter the birth year \\(optional\\):",
        )
    }

    pub fn add_single_enter_username() -> LocalizedText {
        LocalizedText::new(
            "Введите username \\(необязательно\\):",
            "Enter username \\(optional\\):",
        )
    }

    pub fn add_single_enter_notes() -> LocalizedText {
        LocalizedText::new(
            "Введите заметки \\(необязательно\\):",
            "Enter notes \\(optional\\):",
        )
    }

    pub fn add_single_success() -> LocalizedText {
        LocalizedText::new(
            "✅ День рождения успешно добавлен!",
            "✅ Birthday successfully added!",
        )
    }

    // Common messages
    pub fn operation_cancelled() -> LocalizedText {
        LocalizedText::new("❌ Операция отменена.", "❌ Operation cancelled.")
    }

    pub fn invalid_input() -> LocalizedText {
        LocalizedText::new(
            "❌ Неверный ввод. Попробуйте еще раз.",
            "❌ Invalid input. Please try again.",
        )
    }

    pub fn database_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Ошибка базы данных. Попробуйте позже.",
            "❌ Database error. Please try again later.",
        )
    }

    pub fn no_birthdays_found() -> LocalizedText {
        LocalizedText::new("📭 Дни рождения не найдены.", "📭 No birthdays found.")
    }

    // Command descriptions
    pub fn command_help_description() -> LocalizedText {
        LocalizedText::new("Показать это сообщение", "Show this message")
    }

    pub fn command_start_description() -> LocalizedText {
        LocalizedText::new("Начать работу с ботом", "Start working with the bot")
    }

    pub fn command_menu_description() -> LocalizedText {
        LocalizedText::new("Открыть главное меню", "Open main menu")
    }

    pub fn command_stats_description() -> LocalizedText {
        LocalizedText::new("Показать статистику", "Show statistics")
    }

    pub fn command_cancel_description() -> LocalizedText {
        LocalizedText::new("Отменить текущую операцию", "Cancel current operation")
    }

    pub fn command_test_description() -> LocalizedText {
        LocalizedText::new("Тестировать базу данных", "Test database")
    }

    // Add single birthday messages
    pub fn add_single_format_title() -> LocalizedText {
        LocalizedText::new(
            "👤 *Добавление одной записи*\n\nОтправьте сообщение в формате:\n\n`Имя День\\.Месяц\\.Год @username заметки`\n\nПримеры:\n• `Иван 15\\.03\\.1990`\n• `Мария 22\\.12 @maria`\n• `Алексей 05\\.08\\.1985 @alex празднуем вместе`\n\n_Год и заметки необязательны_\\.\n\n🔙 Нажмите кнопку \\\"Назад\\\" для возврата\\.",
            "👤 *Add Single Record*\n\nSend a message in the format:\n\n`Name Day\\.Month\\.Year @username notes`\n\nExamples:\n• `John 15\\.03\\.1990`\n• `Mary 22\\.12 @mary`\n• `Alex 05\\.08\\.1985 @alex celebrating together`\n\n_Year and notes are optional_\\.\n\n🔙 Click the \\\"Back\\\" button to return\\."
        )
    }

    // Add birthday choice messages
    pub fn add_birthday_choice_title() -> LocalizedText {
        LocalizedText::new(
            "➕ *Добавление дня рождения*\n\nВыберите способ добавления:",
            "➕ *Add Birthday*\n\nChoose how to add:",
        )
    }

    pub fn add_single_choice_button() -> LocalizedText {
        LocalizedText::new("👤 Добавить одну запись", "👤 Add Single Record")
    }

    pub fn add_batch_choice_button() -> LocalizedText {
        LocalizedText::new("📄 Загрузить файл", "📄 Upload File")
    }

    // Export messages
    pub fn export_no_data() -> LocalizedText {
        LocalizedText::new(
            "📤 *Экспорт данных*\n\n❌ Нет данных для экспорта\\. Добавьте дни рождения через кнопку \\\"➕ Добавить день рождения\\\"\\.",
            "📤 *Export Data*\n\n❌ No data to export\\. Add birthdays via the \\\"➕ Add Birthday\\\" button\\."
        )
    }

    pub fn export_success(count: i32) -> LocalizedText {
        LocalizedText::new(
            &format!("📤 *Экспорт данных*\n\n✅ Экспортировано {} записей\\.\n\n_Функция экспорта в файл будет доступна в следующей версии_\\.\n\n🔙 Нажмите кнопку \\\"Главное меню\\\" для возврата\\.", count),
            &format!("📤 *Export Data*\n\n✅ Exported {} records\\.\n\n_File export function will be available in the next version_\\.\n\n🔙 Click the \\\"Main Menu\\\" button to return\\.", count)
        )
    }

    pub fn export_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Произошла ошибка при экспорте данных\\.",
            "❌ An error occurred while exporting data\\.",
        )
    }

    // Stats messages
    pub fn stats_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Не удалось получить статистику.",
            "❌ Failed to get statistics.",
        )
    }

    pub fn stats_success(total: i32) -> LocalizedText {
        LocalizedText::new(
            &format!("✅ Статистика: всего {} дней рождений", total),
            &format!("✅ Statistics: {} total birthdays", total),
        )
    }

    // Search messages
    pub fn search_instructions() -> LocalizedText {
        LocalizedText::new(
            "🔍 *Поиск дней рождений*\n\nДля поиска отправьте сообщение с именем человека\\.\n\nНапример: \\\"Иван\\\" или \\\"Мария\\\"\\.\n\n_Поиск работает по частичному совпадению имени_\\.\n\n🔙 Нажмите кнопку \\\"Главное меню\\\" для возврата\\.",
            "🔍 *Search Birthdays*\n\nTo search, send a message with the person's name\\.\n\nFor example: \\\"John\\\" or \\\"Mary\\\"\\.\n\n_Search works by partial name matching_\\.\n\n🔙 Click the \\\"Main Menu\\\" button to return\\."
        )
    }

    // Welcome messages
    pub fn welcome_message() -> LocalizedText {
        LocalizedText::new(
            "👋 Используйте /menu для открытия главного меню или /help для получения справки.",
            "👋 Use /menu to open the main menu or /help for help.",
        )
    }

    pub fn welcome_private() -> LocalizedText {
        LocalizedText::new(
            "👋 Привет! Используйте /menu для открытия главного меню или /help для получения справки.",
            "👋 Hello! Use /menu to open the main menu or /help for help."
        )
    }

    // Birthday added messages
    pub fn birthday_added_success(name: &str, date: &str, extra: &str) -> LocalizedText {
        LocalizedText::new(
            &format!(
                "✅ День рождения добавлен\\!\n\n👤 Имя: {}\n📅 Дата: {}\n{}",
                name, date, extra
            ),
            &format!(
                "✅ Birthday added\\!\n\n👤 Name: {}\n📅 Date: {}\n{}",
                name, date, extra
            ),
        )
    }

    pub fn birthday_already_exists() -> LocalizedText {
        LocalizedText::new(
            "❌ День рождения для этого человека уже существует в этом чате\\.",
            "❌ Birthday for this person already exists in this chat\\.",
        )
    }

    pub fn birthday_add_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Произошла ошибка при добавлении дня рождения\\. Проверьте формат данных\\.",
            "❌ An error occurred while adding the birthday\\. Check the data format\\.",
        )
    }

    // Format error messages
    pub fn format_error(message: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Неверный формат\\! {}\n\nПопробуйте еще раз или нажмите кнопку \\\"🔙 Главное меню\\\" для отмены\\.", message),
            &format!("❌ Invalid format\\! {}\n\nTry again or click the \\\"🔙 Main Menu\\\" button to cancel\\.", message)
        )
    }

    // File upload messages
    pub fn file_upload_instructions() -> LocalizedText {
        LocalizedText::new(
            "📄 Для загрузки файла с днями рождения, отправьте CSV файл\\.",
            "📄 To upload a file with birthdays, send a CSV file\\.",
        )
    }

    // Test messages
    pub fn test_results_title() -> LocalizedText {
        LocalizedText::new(
            "🧪 *Результаты тестирования*\n\n{}",
            "🧪 *Test Results*\n\n{}",
        )
    }

    pub fn test_birthday_added() -> LocalizedText {
        LocalizedText::new(
            "✅ Тестовый день рождения добавлен",
            "✅ Test birthday added",
        )
    }

    pub fn test_birthday_exists() -> LocalizedText {
        LocalizedText::new(
            "ℹ️ Тестовый день рождения уже существует",
            "ℹ️ Test birthday already exists",
        )
    }

    pub fn test_birthday_error(error: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Ошибка добавления: {}", error),
            &format!("❌ Addition error: {}", error),
        )
    }

    pub fn test_stats_error(error: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Ошибка статистики: {}", error),
            &format!("❌ Statistics error: {}", error),
        )
    }

    pub fn test_found_birthdays(count: usize) -> LocalizedText {
        LocalizedText::new(
            &format!("✅ Найдено {} дней рождений для чата", count),
            &format!("✅ Found {} birthdays for chat", count),
        )
    }

    pub fn test_sample_birthday(name: &str, day: i32, month: i32) -> LocalizedText {
        LocalizedText::new(
            &format!("📋 Пример: {} ({}.{})", name, day, month),
            &format!("📋 Example: {} ({}.{})", name, day, month),
        )
    }

    pub fn test_birthday_error_general(error: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Ошибка получения дней рождений: {}", error),
            &format!("❌ Error getting birthdays: {}", error),
        )
    }

    // Operation messages
    pub fn operation_cancelled_success() -> LocalizedText {
        LocalizedText::new("✅ Операция отменена", "✅ Operation cancelled")
    }

    // Error messages
    pub fn general_error(error: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Произошла ошибка\\.\n\nОшибка: `{}`\n\nПопробуйте команду `/test` для диагностики\\.", error),
            &format!("❌ An error occurred\\.\n\nError: `{}`\n\nTry the `/test` command for diagnostics\\.", error)
        )
    }

    // List messages
    pub fn list_empty() -> LocalizedText {
        LocalizedText::new(
            "📋 *Список дней рождений*\n\n❌ Пока нет добавленных дней рождений\\.\n\nПопробуйте добавить первый день рождения через кнопку \\\"➕ Добавить день рождения\\\" или командой `/test` для добавления тестовых данных\\.",
            "📋 *Birthday List*\n\n❌ No birthdays added yet\\.\n\nTry adding the first birthday via the \\\"➕ Add Birthday\\\" button or use `/test` command to add test data\\."
        )
    }

    pub fn list_title() -> LocalizedText {
        LocalizedText::new("📋 *Список дней рождений*\n\n", "📋 *Birthday List*\n\n")
    }

    pub fn list_showing_first_10() -> LocalizedText {
        LocalizedText::new(
            "\n_Показаны первые 10 записей_\\. Добавьте больше через кнопку \\\"➕ Добавить день рождения\\\"\\.",
            "\n_Showing first 10 records_\\. Add more via the \\\"➕ Add Birthday\\\" button\\."
        )
    }

    pub fn search_not_found(query: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("🔍 *Поиск: \\\"{}\\\"*\n\n❌ Ничего не найдено\\.\n\nПопробуйте другой запрос или добавьте день рождения через кнопку \\\"➕ Добавить день рождения\\\"\\.", query),
            &format!("🔍 *Search: \\\"{}\\\"*\n\n❌ Nothing found\\.\n\nTry another query or add a birthday via the \\\"➕ Add Birthday\\\" button\\.", query)
        )
    }

    pub fn search_found(query: &str, count: usize) -> LocalizedText {
        LocalizedText::new(
            &format!(
                "🔍 *Поиск: \\\"{}\\\"*\n\nНайдено {} записей:\n\n",
                query, count
            ),
            &format!(
                "🔍 *Search: \\\"{}\\\"*\n\nFound {} records:\n\n",
                query, count
            ),
        )
    }

    pub fn search_found_count(count: usize) -> LocalizedText {
        if count == 1 {
            LocalizedText::new("Найдена 1 запись.", "Found 1 record.")
        } else {
            LocalizedText::new(
                &format!("Найдено {} записей.", count),
                &format!("Found {} records.", count),
            )
        }
    }

    pub fn fix_errors_and_try_again() -> LocalizedText {
        LocalizedText::new(
            "Пожалуйста, исправьте ошибки и попробуйте снова.",
            "Please fix the errors and try again.",
        )
    }

    pub fn birthday_save_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Ошибка при сохранении дня рождения",
            "❌ Error saving birthday",
        )
    }

    pub fn search_error() -> LocalizedText {
        LocalizedText::new("❌ Ошибка при поиске", "❌ Search error")
    }

    // CSV headers
    pub fn csv_headers() -> LocalizedText {
        LocalizedText::new(
            "Имя,День,Месяц,Год,Username,Заметки",
            "Name,Day,Month,Year,Username,Notes",
        )
    }

    // Validation error messages
    pub fn validation_name_empty() -> LocalizedText {
        LocalizedText::new("Имя не может быть пустым", "Name cannot be empty")
    }

    pub fn validation_name_too_long() -> LocalizedText {
        LocalizedText::new(
            "Имя слишком длинное (максимум 100 символов)",
            "Name is too long (maximum 100 characters)",
        )
    }

    pub fn validation_username_empty() -> LocalizedText {
        LocalizedText::new(
            "Имя пользователя не может быть пустым",
            "Username cannot be empty",
        )
    }

    pub fn validation_username_too_long() -> LocalizedText {
        LocalizedText::new(
            "Имя пользователя слишком длинное (максимум 100 символов)",
            "Username is too long (maximum 100 characters)",
        )
    }

    pub fn validation_username_invalid() -> LocalizedText {
        LocalizedText::new(
            "Имя пользователя может содержать только буквы, цифры и подчеркивания",
            "Username can only contain letters, numbers, and underscores",
        )
    }

    pub fn validation_date_required() -> LocalizedText {
        LocalizedText::new("Дата рождения обязательна", "Date of birth is required")
    }

    pub fn validation_invalid_date() -> LocalizedText {
        LocalizedText::new("Неверный формат даты", "Invalid date")
    }

    pub fn validation_month_range() -> LocalizedText {
        LocalizedText::new(
            "Месяц должен быть от 1 до 12",
            "Month must be between 1 and 12",
        )
    }

    pub fn validation_day_range() -> LocalizedText {
        LocalizedText::new(
            "День должен быть от 1 до 31",
            "Day must be between 1 and 31",
        )
    }

    pub fn validation_invalid_day_for_month(day: i32, month: i32) -> LocalizedText {
        LocalizedText::new(
            &format!("День {} недопустим для месяца {}", day, month),
            &format!("Day {} is invalid for month {}", day, month),
        )
    }

    pub fn validation_year_range() -> LocalizedText {
        LocalizedText::new(
            "Год должен быть от 1900 до 2100",
            "Year must be between 1900 and 2100",
        )
    }

    pub fn validation_notes_too_long() -> LocalizedText {
        LocalizedText::new(
            "Заметки слишком длинные (максимум 500 символов)",
            "Notes are too long (maximum 500 characters)",
        )
    }

    // Rate limit messages
    pub fn rate_limit_retry_minutes_seconds(minutes: i32, seconds: i32) -> LocalizedText {
        LocalizedText::new(
            &format!(
                "Слишком много запросов. Попробуйте снова через {} мин {} сек",
                minutes, seconds
            ),
            &format!(
                "Too many requests. Try again in {} min {} sec",
                minutes, seconds
            ),
        )
    }

    pub fn rate_limit_retry_seconds(seconds: i32) -> LocalizedText {
        LocalizedText::new(
            &format!(
                "Слишком много запросов. Попробуйте снова через {} сек",
                seconds
            ),
            &format!("Too many requests. Try again in {} sec", seconds),
        )
    }

    pub fn rate_limit_blocked_hours_minutes(
        hours: i32,
        minutes: i32,
        reason: &str,
    ) -> LocalizedText {
        LocalizedText::new(
            &format!(
                "Вы заблокированы на {} ч {} мин. Причина: {}",
                hours, minutes, reason
            ),
            &format!(
                "You are blocked for {} h {} min. Reason: {}",
                hours, minutes, reason
            ),
        )
    }

    pub fn rate_limit_blocked_minutes(minutes: i32, reason: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("Вы заблокированы на {} мин. Причина: {}", minutes, reason),
            &format!("You are blocked for {} min. Reason: {}", minutes, reason),
        )
    }

    // Notification template
    pub fn notification_template() -> LocalizedText {
        LocalizedText::new(
            "🎉 Сегодня день рождения у {name}! {username}",
            "🎉 Today is {name}'s birthday! {username}",
        )
    }

    // Format error messages
    pub fn format_insufficient_data() -> LocalizedText {
        LocalizedText::new(
            "Недостаточно данных. Формат: Имя День.Месяц.Год @username заметки",
            "Insufficient data. Format: Name Day.Month.Year @username notes",
        )
    }

    pub fn format_invalid_date() -> LocalizedText {
        LocalizedText::new(
            "Неверный формат даты. Используйте День.Месяц.Год",
            "Invalid date format. Use Day.Month.Year",
        )
    }

    pub fn format_invalid_day() -> LocalizedText {
        LocalizedText::new("Неверный день", "Invalid day")
    }

    pub fn format_invalid_month() -> LocalizedText {
        LocalizedText::new("Неверный месяц", "Invalid month")
    }

    pub fn format_invalid_year() -> LocalizedText {
        LocalizedText::new("Неверный год", "Invalid year")
    }

    // Main menu button
    pub fn main_menu_button() -> LocalizedText {
        LocalizedText::new("🔙 Главное меню", "🔙 Main Menu")
    }

    // Test user data
    pub fn test_user_name() -> LocalizedText {
        LocalizedText::new("Тестовый Пользователь", "Test User")
    }

    pub fn test_user_notes() -> LocalizedText {
        LocalizedText::new("Тестовый день рождения", "Test birthday")
    }

    // Error messages for BotError localization
    pub fn validation_error(message: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Ошибка валидации: {}", message),
            &format!("❌ Validation error: {}", message),
        )
    }

    pub fn authentication_error() -> LocalizedText {
        LocalizedText::new("❌ Ошибка аутентификации.", "❌ Authentication error.")
    }

    pub fn authorization_error() -> LocalizedText {
        LocalizedText::new(
            "❌ У вас недостаточно прав для выполнения этой операции.",
            "❌ You don't have sufficient permissions for this operation.",
        )
    }

    pub fn rate_limit_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Слишком много запросов. Попробуйте повторить позже.",
            "❌ Too many requests. Please try again later.",
        )
    }

    pub fn file_too_large_error(size: usize, max_size: usize) -> LocalizedText {
        LocalizedText::new(
            &format!(
                "❌ Файл слишком большой: {} байт (максимум: {} байт).",
                size, max_size
            ),
            &format!(
                "❌ File too large: {} bytes (max: {} bytes).",
                size, max_size
            ),
        )
    }

    pub fn invalid_file_format_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Неверный формат файла. Поддерживается только JSON.",
            "❌ Invalid file format. Only JSON is supported.",
        )
    }

    pub fn resource_limit_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Превышен лимит ресурсов. Попробуйте уменьшить количество данных.",
            "❌ Resource limit exceeded. Try reducing the amount of data.",
        )
    }

    pub fn not_found_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Запрашиваемые данные не найдены.",
            "❌ Requested data not found.",
        )
    }

    pub fn telegram_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Ошибка Telegram API. Попробуйте позже.",
            "❌ Telegram API error. Please try again later.",
        )
    }

    pub fn external_service_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Ошибка внешнего сервиса. Попробуйте позже.",
            "❌ External service error. Please try again later.",
        )
    }

    pub fn internal_error() -> LocalizedText {
        LocalizedText::new(
            "❌ Произошла внутренняя ошибка. Попробуйте повторить позже.",
            "❌ An internal error occurred. Please try again later.",
        )
    }

    // Common error handling
    pub fn error_with_details(error: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Произошла ошибка\\.\n\nОшибка: `{}`\n\nПопробуйте команду `/test` для диагностики\\.", error),
            &format!("❌ An error occurred\\.\n\nError: `{}`\n\nTry the `/test` command for diagnostics\\.", error)
        )
    }

    pub fn permission_denied_private() -> LocalizedText {
        LocalizedText::new(
            "❌ У вас нет прав для использования этого бота.",
            "❌ You don't have permission to use this bot.",
        )
    }

    pub fn permission_denied_group() -> LocalizedText {
        LocalizedText::new(
            "❌ Только администраторы могут добавлять дни рождения в группах и каналах.",
            "❌ Only administrators can add birthdays in groups and channels.",
        )
    }

    pub fn simple_error(error: &str) -> LocalizedText {
        LocalizedText::new(
            &format!("❌ Ошибка: {}", error),
            &format!("❌ Error: {}", error),
        )
    }

    // Stats-specific messages
    pub fn stats_total_birthdays() -> LocalizedText {
        LocalizedText::new("Всего дней рождений", "Total birthdays")
    }

    pub fn stats_birthdays_today() -> LocalizedText {
        LocalizedText::new("Дней рождений сегодня", "Birthdays today")
    }

    pub fn stats_with_year() -> LocalizedText {
        LocalizedText::new("С указанным годом", "With specified year")
    }
}
