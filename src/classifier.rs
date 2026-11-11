use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardEntryType {
    URL,
    UNKNOWN,
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
                    kind: ClipboardEntryType::URL,
                    content: trimmed.to_string(),
                };
            }
            _ => {}
        }
    }

    ClassifiedEntry {
        kind: ClipboardEntryType::UNKNOWN,
        content: trimmed.to_string(),
    }
}
