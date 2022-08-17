pub mod r09;

use chrono::NaiveDateTime;


use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TelegramMetaInformation {
    pub time: NaiveDateTime,
    pub station: Uuid,
    pub region: i32, // foreign key references regions
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthenticationMeta {
    pub station: Uuid,
    pub token: String,
}
