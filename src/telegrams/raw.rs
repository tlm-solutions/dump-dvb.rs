use super::super::schema::raw_telegrams;
use super::{AuthenticationMeta, GetTelegramType, TelegramMetaInformation, TelegramType};

use std::fmt;
use std::hash::{Hash, Hasher};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;
use uuid::Uuid;

/// Struct for raw bytes of a VDV420 telegram
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawTelegram {
    /// Telegram type, see [`TelegramType`]
    pub telegram_type: TelegramType,
    /// Raw bytes of intercepted telegram
    pub data: Vec<u8>,
}

/// Raw telegram representation in the database
#[derive(
    Deserialize, Serialize, Debug, Queryable, Insertable, Clone, PartialEq, Eq, FieldNamesAsArray,
)]
#[diesel(table_name = raw_telegrams)]
pub struct RawSaveTelegram {
    /// Auto-incrementing primary key for the postgres, should be [`None`] on insert
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    /// Timestamp of when the [`RawTelegram`] was intercepted
    pub time: NaiveDateTime,
    /// UUID of intercepting station
    pub station: Uuid,
    /// Type of the telegram, see [`TelegramType`]
    pub telegram_type: i64,
    /// Raw bytes of intercepted telegram
    pub data: Vec<u8>,
}

/// Struct that attaches station information to a intercepted [`RawTelegram`]. This can be
/// processed further in data-accumulator
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawReceiveTelegram {
    /// Authentication information for the Station. See [`AuthenticationMeta`]
    #[serde(flatten)]
    pub auth: AuthenticationMeta,
    /// Intercepted [`RawTelegram`]
    #[serde(flatten)]
    pub data: RawTelegram,
}

impl GetTelegramType for RawTelegram {
    fn get_type(&self) -> TelegramType {
        self.telegram_type.clone()
    }
}

impl RawSaveTelegram {
    /// Annotates [`RawTelegram`] with metadata, so we can store it in our nice postgres
    pub fn from(telegram: RawTelegram, meta: TelegramMetaInformation) -> RawSaveTelegram {
        RawSaveTelegram {
            id: None,

            time: meta.time,
            station: meta.station,

            telegram_type: telegram.telegram_type as i64,
            data: telegram.data,
        }
    }
}

impl Hash for RawReceiveTelegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Hash for RawTelegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.telegram_type.hash(state);
        self.data.hash(state);
    }
}

impl fmt::Display for RawTelegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type {:?} Raw Data: {:#?}",
            self.telegram_type, self.data,
        )
    }
}
