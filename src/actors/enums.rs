#[allow(dead_code)]
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

pub enum IntensityLevel {
    HIGH,
    LOW,
}

impl IntensityLevel {
    pub fn from_str(s: &str) -> Option<IntensityLevel> {
        match s {
            "high" => Some(IntensityLevel::HIGH),
            "low" => Some(IntensityLevel::LOW),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            IntensityLevel::HIGH => "high",
            IntensityLevel::LOW => "low",
        }
    }
}
