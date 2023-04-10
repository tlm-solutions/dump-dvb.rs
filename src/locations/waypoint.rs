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
    id: u64,
    /// from which data source this waypoint was constructed
    source: WayPointType,
    /// unix time stamp in milliseconds
    time: u64,
    /// region identifier
    region: i64,
    /// latitude
    lat: f64,
    /// longitude
    lon: f64,
    /// line of the vehicle
    line: i32,
    /// run (ger. Kurs, -Laufnummer) of the vehicle
    run: i32,
    /// optional delay the vehicle has
    delayed: Option<f32>,
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
