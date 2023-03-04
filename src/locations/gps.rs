use crate::schema::*;
use serde::{Deserialize, Serialize, Serializer};

// Public Structs
// Only parts relevant for the interpolation are here
/// Gps trackpoint representation used in lofi for Gps data
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Clone, Copy, Debug)]
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
