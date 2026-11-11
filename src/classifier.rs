use serde;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardEntryType {
    Url,
    Unknown,
    Email
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct ClassifiedEntry {
    pub kind: ClipboardEntryType,
    pub content: String,
}

pub fn classify_clipboard_entry(entry: &str) -> ClassifiedEntry {
    let trimmed = entry.trim();

    if let Ok(parsed_url) = Url::parse(trimmed) {
        match parsed_url.scheme() {
            // We can select the protocol here
            "http" | "https" => {
                return ClassifiedEntry {
                    kind: ClipboardEntryType::Url,
                    content: trimmed.to_string(),
                };
            }
            _ => {}
        }
    }

    ClassifiedEntry {
        kind: ClipboardEntryType::Unknown,
        content: trimmed.to_string(),
    }
}
