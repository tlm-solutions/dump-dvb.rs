//!
//! This modul contains structs, exchange formats and implementations for R09 Telegrams.
//!

use crate::management::Station;
use crate::schema::r09_telegrams;
use crate::telegrams::{
    AuthenticationMeta, GetTelegramType, TelegramMetaInformation, TelegramType,
};

use chrono::NaiveDateTime;
use csv;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::Insertable;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;
use uuid::Uuid;

use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use crate::grpc::R09GrpcTelegram;

/// The R09Telegram is the heart piece it hold the raw information from the received
/// radio-telegram. The goal was of this struct is to be the smallest denominator
/// of all different telegram formats (**R09.14**, **R09.16**, **R09.18**).
///
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct R09Telegram {
    /// standard the telegram follows (**R09.14**, **R09.16**, **R09.18**)
    pub r09_type: R09Type,
    /// delay of the vehicle can range from -7 min to +7 mins
    pub delay: Option<i32>,
    /// Unique identifier of a location which can be decomposed into junction, direction and
    /// request_status.
    pub reporting_point: u32,
    /// Identifier of a traffic light.
    pub junction: u32,
    /// Which direction the vehicle wants to drive.
    pub direction: u8,
    /// Enum in which state of registration this vehicle is see
    /// [RequestStatus][crate::locations::RequestStatus] for more information.
    pub request_status: u8,
    /// If the vehicle requests priority or not.
    pub priority: Option<u8>,
    /// TODO: marenz I am not standing up to get the paper now NOO !
    pub direction_request: Option<u8>,
    /// Line (ger. linie) of the vehicle.
    pub line: Option<u32>,
    /// Run Number (ger. Kurs Nummer) of the vehicle.
    pub run_number: Option<u32>,
    /// Number identifing last stop on the track.
    pub destination_number: Option<u32>,
    /// Number of individual carriages. Normally this value is 0 because modern
    /// low-floor vehicles dont have seperate carriages.
    pub train_length: Option<i32>,
    /// Number identifing the vehicle (only in >= R09.16).
    pub vehicle_number: Option<u32>,
    /// Number identifing different operators (only in R09.18).
    pub operator: Option<u8>,
}

/// R09SaveTelegram is how R09Telegrams are saved in the database or csv. Furthermore
/// it is enriched with meta information about the receiver that caught this telegram
/// first or at which time this telegram was transmitted.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    Deserialize,
    Serialize,
    Insertable,
    Associations,
    FieldNamesAsArray,
    Queryable,
)]
#[diesel(table_name = r09_telegrams)]
#[diesel(belongs_to(Station, foreign_key = station))]
pub struct R09SaveTelegram {
    /// Unique Identifier for a telegram.
    #[serde(deserialize_with = "csv::invalid_option")]
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,

    /// Timepoint when the telegram was received this assumes UTC.
    pub time: NaiveDateTime,
    /// UUID of the station that received this telegram.
    pub station: Uuid,

    /// standard the telegram follows (**R09.14**, **R09.16**, **R09.18**)
    #[diesel(deserialize_as = i64)]
    pub r09_type: R09Type,
    #[serde(deserialize_with = "csv::invalid_option")]

    /// delay of the vehicle can range from -7 min to +7 mins
    pub delay: Option<i32>,
    /// Unique identifier of a location which can be decomposed into junction, direction and
    /// request_status.
    pub reporting_point: i32,
    /// Identifier of a traffic light.
    pub junction: i32,
    /// Which direction the vehicle wants to drive.
    pub direction: i16,
    /// Enum in which state of registration this vehicle is see
    /// [RequestStatus][crate::locations::RequestStatus] for more information.
    pub request_status: i16,
    /// If the vehicle requests priority or not.
    pub priority: Option<i16>,
    /// TODO: marenz I am not standing up to get the paper now NOO !
    pub direction_request: Option<i16>,
    /// Line (ger. linie) of the vehicle.
    pub line: Option<i32>,
    /// Run Number (ger. Kurs Nummer) of the vehicle.
    pub run_number: Option<i32>,
    /// Number identifing last stop on the track.
    pub destination_number: Option<i32>,
    /// Number of individual carriages. Normally this value is 0 because modern
    /// low-floor vehicles dont have seperate carriages.
    pub train_length: Option<i32>,
    /// Number identifing the vehicle (only in >= R09.16).
    pub vehicle_number: Option<i32>,
    /// Number identifing different operators (only in R09.18).
    pub operator: Option<i16>,
    /// Region where the telegram was received.
    pub region: i64,
}

/// This Telegram is what the **data-hoarder** service expects when submitting new telegrams.
/// It is enrichted with data for authentication like your secret token or the station identifier.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct R09ReceiveTelegram {
    /// struct that holds all relevant authentication information of a telegram. Most importantly
    /// Station UUID and Station Token which are checked by data-accumulator.
    #[serde(flatten)]
    pub auth: AuthenticationMeta,
    /// R09 Telegram Data
    #[serde(flatten)]
    pub data: R09Telegram,
}

/// Enum for different R09 formats used.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone, AsExpression)]
#[diesel(sql_type = diesel::sql_types::BigInt)]
pub enum R09Type {
    R14 = 14,
    R16 = 16,
    R18 = 18,
}

impl fmt::Display for R09Type {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            R09Type::R14 => formatter.write_str("R09.14"),
            R09Type::R16 => formatter.write_str("R09.16"),
            R09Type::R18 => formatter.write_str("R09.18"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for R09Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct R09TypeVisitor;

        impl<'de> serde::de::Visitor<'de> for R09TypeVisitor {
            type Value = R09Type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or string representing a R09Type")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<R09Type, E> {
                Ok(match s {
                    "R09.14" => R09Type::R14,
                    "R09.16" => R09Type::R16,
                    "R09.18" => R09Type::R18,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Str(s), &self)),
                })
            }

            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<R09Type, E> {
                Ok(match n {
                    14 => R09Type::R14,
                    16 => R09Type::R16,
                    18 => R09Type::R18,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Unsigned(n), &self)),
                })
            }
        }

        deserializer.deserialize_any(R09TypeVisitor)
    }
}

impl Serialize for R09Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        // Serialize the enum as a string.
        serializer.serialize_i16(match *self {
            R09Type::R14 => 14,
            R09Type::R16 => 16,
            R09Type::R18 => 18,
        })
    }
}

impl TryFrom<i64> for R09Type {
    type Error = String;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            14 => Ok(Self::R14),
            16 => Ok(Self::R16),
            18 => Ok(Self::R18),
            _ => Err("No such R09 type!".to_string()),
        }
    }
}

impl FromSql<diesel::sql_types::BigInt, Pg> for R09Type {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> deserialize::Result<Self> {
        //<R09Type as deserialize::FromSql<diesel::sql_types::BigInt, Pg>>::from_sql(bytes).map(|i| R09Type::from(i))
        let v: i64 = i64::from_sql(bytes)?;
        let res: R09Type = v.try_into()?;
        Ok(res)
    }
}

impl ToSql<diesel::sql_types::BigInt, Pg> for R09Type {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            R09Type::R14 => <i64 as ToSql<diesel::sql_types::BigInt, Pg>>::to_sql(&14_i64, out),
            R09Type::R16 => <i64 as ToSql<diesel::sql_types::BigInt, Pg>>::to_sql(&16_i64, out),
            R09Type::R18 => <i64 as ToSql<diesel::sql_types::BigInt, Pg>>::to_sql(&18_i64, out),
        }
    }
}

impl Hash for R09Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            R09Type::R14 => {
                14i32.hash(state);
            }
            R09Type::R16 => {
                16i32.hash(state);
            }
            R09Type::R18 => {
                18i32.hash(state);
            }
        }
    }
}

impl GetTelegramType for R09Telegram {
    fn get_type(&self) -> TelegramType {
        TelegramType::R09
    }
}

impl R09SaveTelegram {
    /// Takes a raw R09Telegram and Meta data to create a R09SaveTelegram which then can be written
    /// to the database.
    pub fn from(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09SaveTelegram {
        R09SaveTelegram {
            id: None,

            time: meta.time,
            station: meta.station,
            r09_type: telegram.r09_type,
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
            train_length: telegram.train_length,
            vehicle_number: telegram.vehicle_number.map(|x| x as i32),
            operator: telegram.operator.map(|x| x as i16),
            region: meta.region,
        }
    }
}

impl Hash for R09ReceiveTelegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl Hash for R09Telegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r09_type.hash(state);
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
            self.r09_type, self.line, self.run_number, self.destination_number, self.request_status
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
        s.serialize_field("r09_type", &self.r09_type)?;

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
    /// Creates a R09GrpcTelegram from a raw R09Telegram and Meta Information.
    pub fn create(telegram: R09Telegram, meta: TelegramMetaInformation) -> R09GrpcTelegram {
        R09GrpcTelegram {
            time: meta.time.timestamp() as u64,
            station: meta.station.to_string(),
            region: meta.region,

            r09_type: telegram.r09_type as i32,
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
            train_length: telegram.train_length,
            vehicle_number: telegram.vehicle_number.map(|x| x as i32),
            operator: telegram.operator.map(|x| x as i32),
        }
    }
}
