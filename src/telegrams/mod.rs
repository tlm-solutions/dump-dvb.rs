pub mod r09;
pub mod raw;

use std::hash::{Hasher, Hash};
use core::fmt::Debug;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use num_derive::FromPrimitive;

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


/// All the different Telegram flawors
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
    C15 = 31
}


impl Hash for TelegramType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.clone() as u8).hash(state);
    }
}

pub trait GetTelegramType {
    fn get_type(self: &Self) -> TelegramType where Self: Sized;
}

