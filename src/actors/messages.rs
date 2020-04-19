use crate::actors::enums::{IntensityLevel, RoutingKey};
use actix::Message;
use serde::{Deserialize, Serialize};

pub struct LoveMessage {
    pub routing_key: RoutingKey,
    pub body: String,
}

impl Message for LoveMessage {
    type Result = Result<(), ()>;
}

// Correspondent message for Love-Event with confession routing-key
#[derive(Serialize, Deserialize)]
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

// Correspondent message for Love-Event with breakup routing-key
#[derive(Serialize, Deserialize)]
pub struct BreakupMessage {
    pub target: String,
    pub user: String,
    pub date: u32,
    pub certainty: bool,
}

impl Message for BreakupMessage {
    type Result = Result<(), ()>;
}
