use crate::mappings::{
    LATIN_TO_YUTF8, YUTF8_TO_LATIN,
    SERBIAN_CYRILLIC_TO_YUTF8, YUTF8_TO_SERBIAN_CYRILLIC,
    MACEDONIAN_TO_YUTF8, YUTF8_TO_MACEDONIAN
};

/// Supported script types for conversion
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScriptType {
    Latin,
    SerbianCyrillic,
    Macedonian,
}

/// Converts a string from UTF-8 to YUTF-8 based on the specified script type
pub fn convert_to_yutf8(text: &str, script: ScriptType) -> String {
    let mapping = match script {
        ScriptType::Latin => &*LATIN_TO_YUTF8,
        ScriptType::SerbianCyrillic => &*SERBIAN_CYRILLIC_TO_YUTF8,
        ScriptType::Macedonian => &*MACEDONIAN_TO_YUTF8,
    };
    
    text.chars()
        .map(|c| *mapping.get(&c).unwrap_or(&c))
        .collect()
}

/// Converts a string from YUTF-8 to UTF-8 based on the specified script type
pub fn convert_from_yutf8(text: &str, script: ScriptType) -> String {
    let mapping = match script {
        ScriptType::Latin => &*YUTF8_TO_LATIN,
        ScriptType::SerbianCyrillic => &*YUTF8_TO_SERBIAN_CYRILLIC,
        ScriptType::Macedonian => &*YUTF8_TO_MACEDONIAN,
    };
    
    text.chars()
        .map(|c| *mapping.get(&c).unwrap_or(&c))
        .collect()
}