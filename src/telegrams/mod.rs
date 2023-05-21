pub mod r09;
pub mod raw;

use std::hash::{Hash, Hasher};

use chrono::NaiveDateTime;
use num_derive::FromPrimitive;
use securefmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Struct that is sent with the telegram itself to **data-accumulator**. It encodes extra
/// information which is not directly contained in the telegram itself.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TelegramMetaInformation {
    /// When the telegram was received.
    #[serde(with = "crate::time_serializer")]
    pub time: NaiveDateTime,
    /// UUID of the Station it was received.
    pub station: Uuid,
    /// Region Information
    pub region: i64,
}

/// Telegram struct with embedded auth information that is sent to **data-accumulator**
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthenticationMeta {
    /// UUID of the station that sent the telegram.
    pub station: Uuid,
    /// Secret Token of the registered station.
    #[sensitive]
    pub token: String,
    /// When the telegram was received.
    #[serde(with = "crate::time_serializer")]
    pub time: NaiveDateTime,
}

#[allow(missing_docs)]
/// All the different Telegram flavors
#[derive(Serialize, Deserialize, FromPrimitive, Debug, Clone)]
pub enum TelegramType {
    R00 = 0,
    R01 = 1,
    R02 = 2,
    R03 = 3,
    R04 = 4,
    R05 = 5,
    R06 = 6,
    R07 = 7,
    R08 = 8,
    /// The actually interesting and currently only telegram which gets properly decoded.
    R09 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
    C00 = 16,
    C01 = 17,
    C02 = 18,
    C03 = 19,
    C04 = 20,
    C05 = 21,
    C06 = 22,
    C07 = 23,
    C08 = 24,
    C09 = 25,
    C10 = 26,
    C11 = 27,
    C12 = 28,
    C13 = 29,
    C14 = 30,
    C15 = 31,
}

impl Hash for TelegramType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.clone() as u8).hash(state);
    }
}

/// Returns [`TelegramType`] of an implemented telegram type.
pub trait GetTelegramType {
    /// Returns [`TelegramType`] of an implemented telegram type.
    fn get_type(&self) -> TelegramType
    where
        Self: Sized;
}
