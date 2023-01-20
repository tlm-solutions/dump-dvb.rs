use crate::locations::R09Types;
use crate::management::Station;

use crate::schema::r09_telegrams;
use crate::telegrams::{
    AuthenticationMeta, GetTelegramType, TelegramMetaInformation, TelegramType,
};

use chrono::NaiveDateTime;
use csv;
use diesel::Insertable;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;
use uuid::Uuid;

use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;

mod tlms {
    tonic::include_proto!("tlms");
}

pub use tlms::receives_telegrams_client::ReceivesTelegramsClient;
pub use tlms::receives_telegrams_server::ReceivesTelegrams;
pub use tlms::receives_telegrams_server::ReceivesTelegramsServer;
pub use tlms::{R09GrpcTelegram, ReturnCode};

/// The R09Telegram is the heart piece it hold the raw information from the received
/// radio-telegram. The goal was of this struct is to be the smallest denominator
/// of all different telegram formats (**R09.14**, **R09.16**, **R09.18**).
///
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct R09Telegram {
    /// standard the telegram follows (**R09.14**, **R09.16**, **R09.18**)
    pub telegram_type: R09Types,
    /// delay of the vehicle can range from -9 min to +9 mins
    pub delay: Option<i32>,
    /// TODO: marenz
    pub reporting_point: u32,
    /// TODO: marenz
    pub junction: u32, //derived from  reporting_point
    pub direction: u8,      //derived from reporting_point
    pub request_status: u8, //derived from reporting_point
    pub priority: Option<u8>,
    pub direction_request: Option<u8>,
    pub line: Option<u32>,
    pub run_number: Option<u32>,
    pub destination_number: Option<u32>,
    pub train_length: Option<u8>,
    pub vehicle_number: Option<u32>,
    pub operator: Option<u8>,
}

/// R09SaveTelegram is how R09Telegrams are saved in the database or csv. Furthermore
/// it is enriched with meta information about the receiver that caught this telegram
/// first or at which time this telegram was transmitted.
#[derive(
    Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Insertable, Associations, FieldNamesAsArray,
)]
#[diesel(table_name = r09_telegrams)]
#[diesel(belongs_to(Station, foreign_key = station))]
pub struct R09SaveTelegram {
    #[serde(deserialize_with = "csv::invalid_option")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,

    pub time: NaiveDateTime,
    pub station: Uuid,

    pub telegram_type: i64,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub delay: Option<i32>,
    pub reporting_point: i32,
    pub junction: i32,       //derived from  reporting_point
    pub direction: i16,      //derived from reporting_point
    pub request_status: i16, //derived from reporting_point
    pub priority: Option<i16>,
    pub direction_request: Option<i16>,
    pub line: Option<i32>,
    pub run_number: Option<i32>,
    pub destination_number: Option<i32>,
    pub train_length: Option<i16>,
    pub vehicle_number: Option<i32>,
    pub operator: Option<i16>,
    pub region: i64,
}

/// This Telegram is what the **data-hoarder** service expects when submitting new telegrams.
/// It is enrichted with data for authentication like your secret token or the station identifier.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct R09ReceiveTelegram {
    #[serde(flatten)]
    pub auth: AuthenticationMeta,

    #[serde(flatten)]
    pub data: R09Telegram,
}

impl GetTelegramType for R09Telegram {
    fn get_type(&self) -> TelegramType {
        TelegramType::R09
    }
}

impl R09SaveTelegram {
    pub fn from(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09SaveTelegram {
        R09SaveTelegram {
            id: None,

            time: meta.time,
            station: meta.station,
            telegram_type: telegram.telegram_type as i64,
            delay: telegram.delay,
            reporting_point: telegram.reporting_point as i32,
            junction: telegram.junction as i32,
            direction: telegram.direction as i16,
            request_status: telegram.request_status as i16,
            priority: telegram.priority.map(|x| x as i16),
            direction_request: telegram.direction_request.map(|x| x as i16),
            line: telegram.line.map(|x| x as i32),
            run_number: telegram.run_number.map(|x| x as i32),
            destination_number: telegram.destination_number.map(|x| x as i32),
            train_length: telegram.train_length.map(|x| x as i16),
            vehicle_number: telegram.vehicle_number.map(|x| x as i32),
            operator: telegram.operator.map(|x| x as i16),
            region: meta.region,
        }
    }

    pub fn from_csv(path: &String) -> Result<Vec<R09SaveTelegram>, csv::Error> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut collection = Vec::new();
        for result in rdr.deserialize() {
            match result {
                Ok(data) => {
                    collection.push(data);
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            }
        }
        Ok(collection)
    }
}

impl Hash for R09ReceiveTelegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Hash for R09Telegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.telegram_type.hash(state);
        self.delay.hash(state);
        self.reporting_point.hash(state);
        self.junction.hash(state);
        self.direction.hash(state);
        self.request_status.hash(state);
        self.priority.hash(state);
        self.direction_request.hash(state);
        self.line.hash(state);
        self.run_number.hash(state);
        self.destination_number.hash(state);
        self.train_length.hash(state);
        self.vehicle_number.hash(state);
        self.operator.hash(state);
    }
}

impl fmt::Display for R09Telegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type {:?} Line {:#?} Run {:#?} Destination {:#?} - {}",
            self.telegram_type,
            self.line,
            self.run_number,
            self.destination_number,
            self.request_status
        )
    }
}

impl Serialize for R09GrpcTelegram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("R09GrpcTelegram", 17)?;

        s.serialize_field("time", &self.time)?;
        s.serialize_field("station", &self.station)?;
        s.serialize_field("region", &self.region)?;
        s.serialize_field("telegram_type", &self.telegram_type)?;

        self.delay
            .map(|value| s.serialize_field("delay", &value).ok());

        s.serialize_field("reporting_point", &self.reporting_point)?;
        s.serialize_field("junction", &self.junction)?;
        s.serialize_field("direction", &self.direction)?;
        s.serialize_field("request_status", &self.request_status)?;

        self.priority
            .map(|value| s.serialize_field("priority", &value).ok());
        self.direction_request
            .map(|value| s.serialize_field("direction_request", &value).ok());
        self.line
            .map(|value| s.serialize_field("line", &value).ok());
        self.run_number
            .map(|value| s.serialize_field("run_number", &value).ok());
        self.destination_number
            .map(|value| s.serialize_field("destination_number", &value).ok());
        self.train_length
            .map(|value| s.serialize_field("train_length", &value).ok());
        self.vehicle_number
            .map(|value| s.serialize_field("vehicle_number", &value).ok());
        self.operator
            .map(|value| s.serialize_field("operator", &value).ok());

        s.end()
    }
}

impl R09GrpcTelegram {
    pub fn from(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09GrpcTelegram {
        R09GrpcTelegram {
            time: meta.time.timestamp() as u64,
            station: meta.station.to_string(),
            region: meta.region,

            telegram_type: telegram.telegram_type as i32,
            delay: telegram.delay,
            reporting_point: telegram.reporting_point as i32,
            junction: telegram.junction as i32,
            direction: telegram.direction as i32,
            request_status: telegram.request_status as i32,
            priority: telegram.priority.map(|x| x as i32),
            direction_request: telegram.direction_request.map(|x| x as i32),
            line: telegram.line.map(|x| x as i32),
            run_number: telegram.run_number.map(|x| x as i32),
            destination_number: telegram.destination_number.map(|x| x as i32),
            train_length: telegram.train_length.map(|x| x as i32),
            vehicle_number: telegram.vehicle_number.map(|x| x as i32),
            operator: telegram.operator.map(|x| x as i32),
        }
    }
}
