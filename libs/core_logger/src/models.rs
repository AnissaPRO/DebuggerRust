use serde::Deserialize;

/// Représente une ligne brute du fichier JSON.
#[derive(Debug, Deserialize)]
pub struct LogRaw {
    pub date: u64,

    #[serde(rename = "containerId")]
    pub container_id: String,

    pub level: String,

    pub message: MessageField,
}

/// Représente les différents formats possibles du champ `message`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MessageField {
    Detailed {
        #[serde(rename = "type")]
        type_: String,

        log: String,
    },

    Simple(String),

    Object(serde_json::Value),
}

/// Structure métier utilisée par l'application.
#[derive(Debug, Clone)]
pub struct ParsedLog {
    pub level: String,

    pub date: String,

    pub container_id: String,

    pub message: String,
}
