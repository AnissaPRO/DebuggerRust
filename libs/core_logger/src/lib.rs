use chrono::{LocalResult, TimeZone, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct LogRaw {
    pub date: u64,
    pub containerId: String,
    pub level: String,
    pub message: MessageField,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageField {
    Detailed {
        #[serde(rename = "type")]
        type_: String,
        log: String,
    },
    Simple(String),
    Object(serde_json::Value), // Sécurisation OWASP : attrape le JSON complexe
}

pub struct ParsedLog {
    pub level: String,
    pub date: String,
    pub container_id: String,
    pub message: String,
}

pub fn parse_log_file<P: AsRef<Path>>(path: P) -> Vec<ParsedLog> {
    let mut logs = Vec::new();

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if let Ok(raw) = serde_json::from_str::<LogRaw>(&line_content) {
                    let log_text = match raw.message {
                        MessageField::Detailed { log, .. } => log,
                        MessageField::Simple(text) => text,
                        MessageField::Object(json_value) => json_value.to_string(),
                    };

                    let bytes = strip_ansi_escapes::strip(&log_text);
                    let clean_message =
                        String::from_utf8(bytes).unwrap_or_else(|_| log_text.clone());

                    let date_formatee = match Utc.timestamp_millis_opt(raw.date as i64) {
                        LocalResult::Single(dt) => dt.format("%d/%m/%Y %H:%M:%S").to_string(),
                        LocalResult::Ambiguous(dt1, _) => {
                            dt1.format("%d/%m/%Y %H:%M:%S").to_string()
                        }
                        LocalResult::None => "Date invalide".to_string(),
                    };

                    logs.push(ParsedLog {
                        level: raw.level,
                        date: date_formatee,
                        container_id: raw.containerId,
                        message: clean_message,
                    });
                }
            }
        }
    }
    logs
}

// ================= HARNAIS DE TEST UNITAIRE (Critère C2.2.2) =================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_message_simple() {
        let json_line =
            r#"{"date":1776414870130,"containerId":"totem","level":"info","message":"Démarrage"}"#;
        let parsed: Result<LogRaw, _> = serde_json::from_str(json_line);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parsing_message_objet_complexe() {
        let json_line = r#"{"date":1776414870130,"containerId":"totem","level":"debug","message":{"type":"__myway-response__","requestId":null}}"#;
        let parsed: Result<LogRaw, _> = serde_json::from_str(json_line);
        assert!(parsed.is_ok());
    }
}
