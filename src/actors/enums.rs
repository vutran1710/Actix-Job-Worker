use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum RoutingKey {
    LOVE,
    HATE,
}

impl RoutingKey {
    pub fn from_str(s: &str) -> Option<RoutingKey> {
        match s {
            "love-you" => Some(RoutingKey::LOVE),
            "hate-you" => Some(RoutingKey::HATE),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RoutingKey::LOVE => "love-you",
            RoutingKey::HATE => "hate-you",
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntensityLevel {
    HIGH(String),
    LOW(String),
}

impl IntensityLevel {
    pub fn from_str(s: &str) -> Option<IntensityLevel> {
        match s {
            "high" => Some(IntensityLevel::HIGH("high".to_string())),
            "low" => Some(IntensityLevel::LOW("low".to_string())),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            IntensityLevel::HIGH(_) => "high",
            IntensityLevel::LOW(_) => "low",
        }
    }
}
