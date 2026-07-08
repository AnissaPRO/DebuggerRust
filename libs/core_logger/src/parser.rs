use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{clean_message, format_timestamp, LogRaw, LoggerError, MessageField, ParsedLog};

/// Lit un fichier de journaux et retourne la liste des logs analysés.
///
/// Chaque ligne du fichier est supposée contenir un objet JSON.
///
/// Les lignes invalides sont ignorées afin de poursuivre la lecture du reste
/// du fichier.
pub fn parse_log_file<P: AsRef<Path>>(path: P) -> Result<Vec<ParsedLog>, LoggerError> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut logs = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let raw = match serde_json::from_str::<LogRaw>(&line) {
            Ok(log) => log,

            // Une ligne invalide ne bloque pas toute la lecture
            Err(_) => continue,
        };

        let message = match raw.message {
            MessageField::Detailed { log, .. } => log,

            MessageField::Simple(text) => text,

            MessageField::Object(value) => value.to_string(),
        };

        logs.push(ParsedLog {
            level: raw.level,
            date: format_timestamp(raw.date),
            container_id: raw.container_id,
            message: clean_message(&message),
        });
    }

    Ok(logs)
}
