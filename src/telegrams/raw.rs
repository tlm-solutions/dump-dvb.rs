use super::super::schema::raw_telegrams;
use super::{AuthenticationMeta, GetTelegramType, TelegramMetaInformation, TelegramType};

use std::fmt;
use std::hash::{Hash, Hasher};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawTelegram {
    pub telegram_type: TelegramType,
    pub data: Vec<u8>,
}

#[derive(
    Deserialize, Serialize, Debug, Queryable, Insertable, Clone, PartialEq, Eq, FieldNamesAsArray,
)]
#[diesel(table_name = raw_telegrams)]
pub struct RawSaveTelegram {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,

    pub time: NaiveDateTime,
    pub station: Uuid,

    pub telegram_type: i64,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawReceiveTelegram {
    #[serde(flatten)]
    pub auth: AuthenticationMeta,

    #[serde(flatten)]
    pub data: RawTelegram,
}

impl GetTelegramType for RawTelegram {
    fn get_type(&self) -> TelegramType {
        self.telegram_type.clone()
    }
}

impl RawSaveTelegram {
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
