//! This module defines structs and enum working with waypoints

use crate::grpc::GrpcWaypoint;
use serde::{Deserialize, Serialize};

/// this enum tell the waypoint which source they came from
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WayPointType {
    /// Source is unknown
    UnknownSource = 0,
    /// position deduced from received r09 telegram
    R09Telegram = 1,
    /// position submitted via trekkie service
    TrekkieGPS = 2,
}

impl From<i32> for WayPointType {
    fn from(number: i32) -> Self {
        match number {
            x if x == WayPointType::R09Telegram as i32 => WayPointType::R09Telegram,
            x if x == WayPointType::TrekkieGPS as i32 => WayPointType::TrekkieGPS,
            _ => WayPointType::UnknownSource,
        }
    }
}

/// Uniform vehicle update struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Waypoint {
    /// unique identifier of this waypoint
    pub id: u64,
    /// from which data source this waypoint was constructed
    pub source: WayPointType,
    /// unix time stamp in milliseconds
    pub time: u64,
    /// region identifier
    pub region: i64,
    /// latitude
    pub lat: f64,
    /// longitude
    pub lon: f64,
    /// line of the vehicle
    pub line: i32,
    /// run (ger. Kurs, -Laufnummer) of the vehicle
    pub run: i32,
    /// optional delay the vehicle has
    pub delayed: Option<f32>,
}

impl From<GrpcWaypoint> for Waypoint {
    fn from(waypoint: GrpcWaypoint) -> Self {
        Waypoint {
            id: waypoint.id,
            source: WayPointType::from(waypoint.source),
            time: waypoint.time,
            region: waypoint.region,
            lat: waypoint.lat,
            lon: waypoint.lon,
            line: waypoint.line,
            run: waypoint.run,
            delayed: waypoint.delayed,
        }
    }
}
