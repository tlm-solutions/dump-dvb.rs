use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Only parts relevant for the interpolation are here
/// Gps trackpoint representation used in lofi for Gps data
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = gps_points)]
pub struct GpsPoint {
    /// primary key bigserial
    pub id: i64,
    // foreign key referencing the corresponding trekkie run
    pub trekkie_run: i64,
    /// point's timestamp
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

/// struct for inserting a gps point into the database and utilise
/// the auto increment functions from postgres
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = gps_points)]
pub struct InsertGpsPoint {
    /// primary key bigserial
    pub id: Option<i64>,
    // foreign key referencing the corresponding trekkie run
    pub trekkie_run: i64,
    /// point's timestamp
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
