pub mod graph;
mod tests;

use chrono::prelude::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;

// INCREMENT ME ON ANY BREAKING CHANGE!!!!11111one
/// Version of the json shcema used.
const SCHEMA: &str = "1";

/// Enum for different telegram format
#[derive(Debug, PartialEq, Clone)]
pub enum R09Types {
    R14 = 14,
    R16 = 16,
    R18 = 18,
}

/// There are 4 different telegrams which can be send from one location
/// the first one is sent approximetly 150m before the traffic light the registration
/// telegram is send when the vehicle reaces the traffic light and deregistration is send
/// when the bus leaves the stop.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RequestStatus {
    PreRegistration = 0,
    Registration = 1,
    DeRegistration = 2,
    DoorClosed = 3,
}

/// Meta inforamtion about region which then can be used to configure radio receivers
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RegionMetaInformation {
    /// Frequency in Hz
    pub frequency: Option<u64>,
    pub city_name: Option<String>,
    /// Type of R09 telegram
    pub type_r09: Option<R09Types>,
    /// Latitude of the Region, degrees
    pub lat: Option<f64>,
    /// Longitude of the Region, degrees
    pub lon: Option<f64>,
}

lazy_static! {
    #[deprecated(note="REGION_META_MAP is deprecated in favour of using API directly")]
    #[derive(Debug)]
    /// lazy_static map of a region number to `RegionMetaInformation` struct for this region
    pub static ref REGION_META_MAP: HashMap<i64, RegionMetaInformation> = HashMap::from([
        (
            0_i64,
            RegionMetaInformation {
                frequency: Some(170795000),
                city_name: Some(String::from("Dresden")),
                type_r09: Some(R09Types::R16),
                lat: Some(51.05),
                lon: Some(13.74),
            }
        ),
        (
            1_i64,
            RegionMetaInformation {
                frequency: Some(153850000),
                city_name: Some(String::from("Chemnitz")),
                type_r09: Some(R09Types::R16),
                lat: Some(50.82),
                lon: Some(12.92),
            }
        ),
        (
            2_i64,
            RegionMetaInformation {
                frequency: Some(170450000),
                city_name: Some(String::from("Berlin")),
                type_r09: None,
                lat: Some(52.52),
                lon: Some(13.41),
            }
        ),
        (
            3_i64,
            RegionMetaInformation {
                frequency: Some(155630000),
                city_name: Some(String::from("Duesseldorf")),
                type_r09: None,
                lat: Some(50.78),
                lon: Some(6.08),
            }
        ),
        (
            4_i64,
            RegionMetaInformation {
                frequency: Some(150910000),
                city_name: Some(String::from("Hannover")),
                type_r09: Some(R09Types::R14),
                lat: Some(52.38),
                lon: Some(9.73),
            }
        ),
        (
            5_i64,
            RegionMetaInformation {
                frequency: Some(152930000),
                city_name: Some(String::from("Karlsruhe")),
                type_r09: None,
                lat: None,
                lon: None,
            }
        ),
        (
            6_i64,
            RegionMetaInformation {
                frequency: Some(151030000),
                city_name: Some(String::from("Moenchengladbach")),
                type_r09: None,
                lat: None,
                lon: None,
            }
        ),
        (
            7_i64,
            RegionMetaInformation {
                frequency: Some(150827500),
                city_name: Some(String::from("Münster")),
                type_r09: None,
                lat: Some(51.96),
                lon: Some(7.63),
            }
        ),
        (
            8_i64,
            RegionMetaInformation {
                frequency: Some(152930000),
                city_name: Some(String::from("Ulm")),
                type_r09: None,
                lat: None,
                lon: None,
            }
        ),
        (
            9_i64,
            RegionMetaInformation {
                frequency: Some(152850000),
                city_name: Some(String::from("Region-Hannover")),
                type_r09: None,
                lat: None,
                lon: None,
            }
        ),
        (
            10_i64,
            RegionMetaInformation {
                frequency: Some(150827500),
                city_name: Some(String::from("Aachen")),
                type_r09: None,
                lat: Some(50.78),
                lon: Some(6.08),
            }
        ),
    ]);
}

/// Structure containing the coordinates and any extra JSON value for specific report location
/// `meldepunkt`
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ReportLocation {
    /// Latitude of the report location
    pub lat: f64,
    /// Longitude of the report location
    pub lon: f64,
    /// any extra data, as long as this is a valid `serde_json::Value`. Currently only used for
    /// epsg3857.
    pub properties: serde_json::Value,
}

/// Hash map of a report location ID to the `ReportLocation` struct
pub type RegionReportLocations = HashMap<i32, ReportLocation>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct DocumentMeta {
    schema_version: String,
    date: DateTime<Utc>,
    generator: Option<String>,
    generator_version: Option<String>,
}

/// Struct that deserializes directly into locations.json, the main source of transmission location
/// data across the project
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LocationsJson {
    /// meta information about the document, e.g. schema version, and how it was generated.
    document: DocumentMeta,
    /// Hash map of a region number to the `RegionReportLocations`
    pub data: HashMap<i64, RegionReportLocations>,
    /// Hash map of a region number to the meta information about this region
    pub meta: HashMap<i64, RegionMetaInformation>,
}

impl LocationsJson {
    /// Deserialzes file into `LocationsJson`
    pub fn from_file(file: &str) -> Result<LocationsJson, serde_json::error::Error> {
        let data = fs::read_to_string(file).expect("could not read LocationsJson file!");
        serde_json::from_str(&data)
    }

    /// Creates the LocationsJson struct form the hashmaps for data and meta fields, while taking
    /// care of properly filling out the meta private field.
    pub fn construct(
        data: HashMap<i64, RegionReportLocations>,
        meta: HashMap<i64, RegionMetaInformation>,
        generator: Option<String>,
        generator_version: Option<String>,
    ) -> LocationsJson {
        LocationsJson {
            document: DocumentMeta {
                schema_version: String::from(SCHEMA),
                date: chrono::Utc::now(),
                generator,
                generator_version,
            },
            data,
            meta,
        }
    }

    /// Serialises `LocationsJson` to json file. If file exists - silently overwrites it
    pub fn write(&self, file: &str) {
        fs::remove_file(file).ok();
        let mut output = File::create(file).expect("cannot create or open file!");

        let json_data = serde_json::to_string_pretty(&self).expect("cannot serialize structs!");

        output
            .write_all(json_data.as_bytes())
            .expect("cannot write to file!");
    }

    /// Populates document metainformation
    pub fn populate_meta(&mut self, generator: Option<String>, generator_version: Option<String>) {
        self.document = DocumentMeta {
            schema_version: String::from(SCHEMA),
            date: chrono::Utc::now(),
            generator,
            generator_version,
        };
    }
}

impl<'de> serde::Deserialize<'de> for R09Types {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct R09TypesVisitor;

        impl<'de> serde::de::Visitor<'de> for R09TypesVisitor {
            type Value = R09Types;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or string representing a R09Type")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<R09Types, E> {
                Ok(match s {
                    "R09.14" => R09Types::R14,
                    "R09.16" => R09Types::R16,
                    "R09.18" => R09Types::R18,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Str(s), &self)),
                })
            }

            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<R09Types, E> {
                Ok(match n {
                    14 => R09Types::R14,
                    16 => R09Types::R16,
                    18 => R09Types::R18,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Unsigned(n), &self)),
                })
            }
        }

        deserializer.deserialize_any(R09TypesVisitor)
    }
}

impl Serialize for R09Types {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        // Serialize the enum as a string.
        serializer.serialize_i16(match *self {
            R09Types::R14 => 14,
            R09Types::R16 => 16,
            R09Types::R18 => 18,
        })
    }
}

impl fmt::Display for R09Types {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            R09Types::R14 => formatter.write_str("R09.14"),
            R09Types::R16 => formatter.write_str("R09.16"),
            R09Types::R18 => formatter.write_str("R09.18"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for RequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RequestStatusVisitor;

        impl<'de> serde::de::Visitor<'de> for RequestStatusVisitor {
            type Value = RequestStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or string representing a R09Type")
            }
            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<RequestStatus, E> {
                Ok(match n {
                    0 => RequestStatus::PreRegistration,
                    1 => RequestStatus::Registration,
                    2 => RequestStatus::DeRegistration,
                    3 => RequestStatus::DoorClosed,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Unsigned(n), &self)),
                })
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<RequestStatus, E> {
                Ok(match s {
                    "pre_registration" => RequestStatus::PreRegistration,
                    "registration" => RequestStatus::Registration,
                    "de_registration" => RequestStatus::DeRegistration,
                    "door_close" => RequestStatus::DoorClosed,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Str(s), &self)),
                })
            }
        }
        deserializer.deserialize_any(RequestStatusVisitor)
    }
}

impl Serialize for RequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        // Serialize the enum as a string.
        serializer.serialize_i16(match *self {
            RequestStatus::PreRegistration => 0,
            RequestStatus::Registration => 1,
            RequestStatus::DeRegistration => 2,
            RequestStatus::DoorClosed => 3,
        })
    }
}

impl ReportLocation {
    /// Updates property field with epsg3857 coordinates, calculated from `loc` and `lon` fileds of
    /// the struct. If field doesn't exist, creates it.
    pub fn update_epsg3857(&mut self) {
        // Convert the coords to pseudo-mercator (epsg3857)
        const EARTH_RADIUS_M: f64 = 6_378_137_f64;
        let x = EARTH_RADIUS_M * self.lon.to_radians();
        let y =
            ((self.lat.to_radians() / 2. + std::f64::consts::PI / 4.).tan()).ln() * EARTH_RADIUS_M;

        // serialize value
        if let Ok(epsg_val) = serde_json::from_str(&format!("{{ \"x\":{x}, \"y\":{y} }}")) {
            self.properties["epsg3857"] = epsg_val;
        } else {
            eprintln!("epsg3857 property update skipped: Could not serialize {x} and {y} into json Value!");
        }
    }
}

impl TryFrom<i16> for RequestStatus {
    type Error = ();
    /// converts integer to a proper enum value
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RequestStatus::PreRegistration),
            1 => Ok(RequestStatus::Registration),
            2 => Ok(RequestStatus::DeRegistration),
            3 => Ok(RequestStatus::DoorClosed),
            _ => Err(()),
        }
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for R09Types {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            R09Types::R14 => {
                14i32.hash(state);
            }
            R09Types::R16 => {
                16i32.hash(state);
            }
            R09Types::R18 => {
                18i32.hash(state);
            }
        }
    }
}
