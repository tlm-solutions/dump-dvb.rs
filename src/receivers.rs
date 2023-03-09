use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// This is the format of the configuration file that is fed to **telgram-decoder**.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RadioReceiver {
    /// UUID of the station, which can be taken from [`Station`](crate::management::Station).
    pub id: Uuid,
    /// Name of the station, which can be taken from [`Station`](crate::management::Station).
    pub name: String,
    /// Region of the station, which can be taken from [`Station`](crate::management::Station).
    pub region: u32,
    /// Latitude of the station, which can be taken from [`Station`](crate::management::Station).
    pub lat: f64,
    /// Longitude of the station, which can be taken from [`Station`](crate::management::Station).
    pub lon: f64,
}
