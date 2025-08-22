use teloxide::RequestError;

/// Common confirmation handler
pub async fn handle_confirmation_response(
    text: &str,
    on_confirm: impl std::future::Future<Output = Result<(), RequestError>>,
    on_cancel: impl std::future::Future<Output = Result<(), RequestError>>,
    on_invalid: impl std::future::Future<Output = Result<(), RequestError>>,
) -> Result<(), RequestError> {
    match text.to_lowercase().as_str() {
        "да" | "yes" | "подтвердить" | "confirm" => on_confirm.await,
        "нет" | "no" | "отмена" | "cancel" => on_cancel.await,
        _ => on_invalid.await,
    }
}
