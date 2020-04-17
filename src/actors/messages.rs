use crate::actors::enums::{IntensityLevel, RoutingKey};
use actix::Message;

pub struct LoveMessage {
    pub routing_key: RoutingKey,
    pub body: String,
}

impl Message for LoveMessage {
    type Result = Result<(), ()>;
}

// Correspondent message for Love-Event with confession routing-key
pub struct ConfessionMessage {
    pub target: String,
    pub user: String,
    pub intensity_level: IntensityLevel,
    pub date: u32,
    pub certainty: f32,
}

impl Message for ConfessionMessage {
    type Result = Result<(), ()>;
}
