//! This module holds replresentations for geolocation data used all over the TLMS services

use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Gps trackpoint representation used in database.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = gps_points)]
pub struct GpsPoint {
    /// primary key bigserial
    pub id: i64,
    /// foreign key referencing the corresponding trekkie run
    pub trekkie_run: Uuid,
    /// point's timestamp
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub timestamp: NaiveDateTime,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Optional elevation
    pub elevation: Option<f64>,
    /// Optional GPS accuracy
    pub accuracy: Option<f64>,
    /// Optional GPS vertical accuracy
    pub vertical_accuracy: Option<f64>,
    /// Optional GPS bearing (aka angle of the compas)
    pub bearing: Option<f64>,
    /// Optional speed
    pub speed: Option<f64>,
}

/// Struct for inserting a gps point into the database and utilise the auto increment functions
/// from postgres.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = gps_points)]
pub struct InsertGpsPoint {
    /// primary key bigserial
    pub id: Option<i64>,
    /// foreign key referencing the corresponding trekkie run
    pub trekkie_run: Uuid,
    /// point's timestamp
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub timestamp: NaiveDateTime,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Optional elevation
    pub elevation: Option<f64>,
    /// Optional GPS accuracy
    pub accuracy: Option<f64>,
    /// Optional GPS vertical accuracy
    pub vertical_accuracy: Option<f64>,
    /// Optional GPS bearing (aka angle of the compas)
    pub bearing: Option<f64>,
    /// Optional speed
    pub speed: Option<f64>,
}

impl From<GpsPoint> for InsertGpsPoint {
    fn from(val: GpsPoint) -> Self {
        Self {
            id: Some(val.id),
            trekkie_run: val.trekkie_run,
            timestamp: val.timestamp,
            lat: val.lat,
            lon: val.lon,
            elevation: val.elevation,
            accuracy: val.accuracy,
            vertical_accuracy: val.vertical_accuracy,
            bearing: val.bearing,
            speed: val.speed,
        }
    }
}
