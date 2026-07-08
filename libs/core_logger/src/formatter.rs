use chrono::{LocalResult, TimeZone, Utc};

/// Supprime les séquences ANSI présentes dans les journaux.
pub fn clean_message(message: &str) -> String {
    let bytes = strip_ansi_escapes::strip(message);

    String::from_utf8(bytes).unwrap_or_else(|_| message.to_string())
}

/// Convertit un timestamp Unix en date lisible.
pub fn format_timestamp(timestamp: u64) -> String {
    match Utc.timestamp_millis_opt(timestamp as i64) {
        LocalResult::Single(dt) => dt.format("%d/%m/%Y %H:%M:%S").to_string(),

        LocalResult::Ambiguous(dt, _) => dt.format("%d/%m/%Y %H:%M:%S").to_string(),

        LocalResult::None => String::from("Date invalide"),
    }
}
