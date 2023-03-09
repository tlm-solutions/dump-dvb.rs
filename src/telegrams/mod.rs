pub mod r09;
pub mod raw;

use core::fmt::Debug;
use std::hash::{Hash, Hasher};

use chrono::NaiveDateTime;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Struct that is send with the telegram itself to **data-accumulator** which encodes mode
/// information which are not directly contained in the telegram itself.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TelegramMetaInformation {
    /// When the telegram was received.
    pub time: NaiveDateTime,
    /// UUID of the Station it was received.
    pub station: Uuid,
    /// Region Information
    pub region: i64,
}

/// Struct that is send with the telegram itself to **data-accumulator** which contains 
/// authentication information.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthenticationMeta {
    /// UUID of the station that send the telegram.
    pub station: Uuid,
    /// Secret Token of the previosly declared station.
    pub token: String,
    /// When the telegram was received.
    pub time: NaiveDateTime,
}

/// All the different Telegram flawors
#[derive(Serialize, Deserialize, FromPrimitive, Debug, Clone)]
pub enum TelegramType {
    /// ditto
    R00 = 0,
    /// ditto
    R01 = 1,
    /// ditto
    R02 = 2,
    /// ditto
    R03 = 3,
    /// ditto
    R04 = 4,
    /// ditto
    R05 = 5,
    /// ditto
    R06 = 6,
    /// ditto
    R07 = 7,
    /// ditto
    R08 = 8,
    /// The actually interesting and currently only telegram which gets properly decoded.
    R09 = 9,
    /// ditto
    R10 = 10,
    /// ditto
    R11 = 11,
    /// ditto
    R12 = 12,
    /// ditto
    R13 = 13,
    /// ditto
    R14 = 14,
    /// ditto
    R15 = 15,
    /// ditto
    C00 = 16,
    /// ditto
    C01 = 17,
    /// ditto
    C02 = 18,
    /// ditto
    C03 = 19,
    /// ditto
    C04 = 20,
    /// ditto
    C05 = 21,
    /// ditto
    C06 = 22,
    /// ditto
    C07 = 23,
    /// ditto
    C08 = 24,
    /// ditto
    C09 = 25,
    /// ditto
    C10 = 26,
    /// ditto
    C11 = 27,
    /// ditto
    C12 = 28,
    /// ditto
    C13 = 29,
    /// ditto
    C14 = 30,
    /// ditto
    C15 = 31,
}

impl Hash for TelegramType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.clone() as u8).hash(state);
    }
}

pub trait GetTelegramType {
    fn get_type(&self) -> TelegramType
    where
        Self: Sized;
}
