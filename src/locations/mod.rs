mod tests;

use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;

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
#[derive(Debug, PartialEq, Clone)]
pub enum RequestStatus {
    PreRegistration = 0,
    Registration = 1,
    DeRegistration = 2,
    DoorClosed = 3,
}

/// This is used in stops json to define accurate positions
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransmissionPosition {
    #[serde(alias = "DHID")]
    pub dhid: Option<String>,
    pub name: Option<String>,
    pub request_status: RequestStatus,
    pub direction: i16,
    pub lat: f64,
    pub lon: f64,
}

/// Meta inforamtion about region which then can be used to configure radio receivers
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RegionMetaInformation {
    pub frequency: Option<u64>,
    pub city_name: Option<String>,
    pub type_r09: Option<R09Types>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

//pub enum dumpDvbFormat {
//    junction_lists,
//    reporting_points
//}

/// Meta infomration about stops json
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[deprecated()]
pub struct DocumentMetaInformation {
    pub schema_version: String,
    pub date: DateTime<Utc>,
    pub generator: Option<String>,
    pub generator_version: Option<String>,
}

#[deprecated()]
pub type RegionalTransmissionPositions = HashMap<i32, Vec<TransmissionPosition>>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[deprecated()]
pub struct InterRegional {
    pub document: DocumentMetaInformation,
    pub data: HashMap<i32, RegionalTransmissionPositions>,
    pub meta: HashMap<i32, RegionMetaInformation>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ReportLocation {
    pub lat: f64,
    pub lon: f64,
    pub properties: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LineSegment {
    pub historical_time: u32, // time in seconds
    pub next_reporting_point: i32, // reporting_point
    pub positions: Vec<(f64, f64)>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Segments {
    // direction (maybe arbitery) -> reporting_point
    pub segments: HashMap<u32, LineSegment>,
}

pub type RegionReportLocations = HashMap<i32, ReportLocation>;

const SCHEMA: &str = "1";
#[derive(Serialize, Deserialize)]
struct DocumentMeta {
    schema_version: String,
    date: DateTime<Utc>,
    generator: Option<String>,
    generator_version: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LocationsJson {
    document: DocumentMeta,
    pub data: HashMap<i32, RegionReportLocations>,
    pub meta: HashMap<i32, RegionMetaInformation>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Region {
    pub traffic_lights: RegionalTransmissionPositions,
    pub meta: RegionMetaInformation,
}

impl LocationsJson {
    pub fn from_file(file: &str) -> Result<LocationsJson, serde_json::error::Error> {
        let data = fs::read_to_string(file).expect("could not read LocationsJson file!");
        serde_json::from_str(&data)
    }

    // FIXME
    pub fn construct(
        data: HashMap<i32, RegionReportLocations>,
        meta: HashMap<i32, RegionMetaInformation>,
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
            data: data,
            meta: meta,
        }
    }

    pub fn write(&self, file: &str) {
        // FIXME proper logic instead of silent overwrite
        fs::remove_file(file).ok();
        let mut output = File::create(file).expect("cannot create or open file!");

        let json_data = serde_json::to_string_pretty(&self).expect("cannot serialize structs!");

        output
            .write_all(json_data.as_bytes())
            .expect("cannot write to file!");
    }

    fn populate_meta(&mut self, generator: Option<String>, generator_version: Option<String>) {
        self.document = DocumentMeta {
            schema_version: String::from(SCHEMA),
            date: chrono::Utc::now(),
            generator,
            generator_version,
        };
    }

    fn populate_meta_region() {
        todo!();
    }
}

impl InterRegional {
    pub fn from(file: &str) -> Option<InterRegional> {
        let data = fs::read_to_string(file);

        if data.is_err() {
            return None;
        }

        serde_json::from_str(&data.unwrap()).ok()
    }

    pub fn write(&self, file: &str) {
        fs::remove_file(file).ok();
        let mut output = File::create(file).expect("cannot create or open file!");

        let json_data = serde_json::to_string_pretty(&self).expect("cannot serialize structs!");

        output
            .write_all(json_data.as_bytes())
            .expect("cannot write to file!");
    }

    pub fn extract(&self, region_id: &i32) -> Option<Region> {
        let data = self.data.get(region_id);
        let meta = self.meta.get(region_id);

        if data.is_none() || meta.is_none() {
            return None;
        }

        Some(Region {
            traffic_lights: data.unwrap().clone(),
            meta: meta.unwrap().clone(),
        })
    }

    pub fn look_up(
        &self,
        region_id: &i32,
        traffic_light: &i32,
    ) -> Option<Vec<TransmissionPosition>> {
        match self.data.get(region_id) {
            Some(region) => {
                return region.get(traffic_light).map(|x| x.clone());
            }
            None => None,
        }
    }

    pub fn get_approximate_position(
        &self,
        region_id: &i32,
        traffic_light: &i32,
    ) -> Option<TransmissionPosition> {
        let stop_list = self.look_up(region_id, traffic_light);

        match stop_list {
            Some(possbile_stations) => {
                if possbile_stations.len() == 0 {
                    return None;
                }

                let selected_position = possbile_stations[0].clone();

                for position in possbile_stations {
                    if position.request_status == RequestStatus::DoorClosed {
                        return Some(position);
                    }
                }

                Some(selected_position.clone())
            }
            None => None,
        }
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

impl RequestStatus {
    pub fn from_i16(value: i16) -> Option<RequestStatus> {
        match value {
            0 => Some(RequestStatus::PreRegistration),
            1 => Some(RequestStatus::Registration),
            2 => Some(RequestStatus::DeRegistration),
            3 => Some(RequestStatus::DoorClosed),
            _ => None,
        }
    }
}

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
