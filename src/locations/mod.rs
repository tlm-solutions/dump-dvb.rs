pub mod gps;
pub mod region;
mod tests;
pub mod waypoint;

use crate::schema::*;

/// Version of the [`LocationsJson`] shcema used.
pub const SCHEMA: &str = "2"; // INCREMENT ME ON ANY BREAKING CHANGE!!!!11111one

/// Default region cache lifetime in seconds (24h)
pub const REGION_CACHE_EXPIRATION: i64 = 24 * 60 * 60;
/// Name for a cache file
pub const REGION_CACHE_FILE: &str = "region_cache.json";
/// maximum distance in meters
pub const SANE_INTERPOLATION_DISTANCE: i32 = 50;
/// Mean earth radius, required for calcuation of distances between the GPS points
pub const MEAN_EARTH_RADIUS: u32 = 6_371_000;

/// This struct is used to query R09 telegram transmission positions from the database. Every entry
/// corresponds to unique transmission location, that is inferred over multiple measurements. For
/// raw per-measurement data see [`TransmissionLocationRaw`]
#[derive(Debug, Clone, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = r09_transmission_locations)]
pub struct TransmissionLocation {
    /// Primary key
    pub id: i64,
    /// ID of the region where telegram was transmitted
    pub region: i64,
    /// Reporting Point inside the r09 telegram (*meldepunkt*) ID
    pub reporting_point: i32,
    /// Report location latitude
    pub lat: f64,
    /// Report location longitude
    pub lon: f64,
    /// If this transmission postion inserted from absolute data, and all the inference for it
    /// should be ignored
    pub ground_truth: bool,
}

/// This struct is used to insert R09 telegram transmission positions to the database. Every entry
/// corresponds to unique transmission location, that is inferred over multiple measurements. For
/// raw per-measurement data see [`InsertTransmissionLocationRaw`]
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = r09_transmission_locations)]
pub struct InsertTransmissionLocation {
    /// Primary key. During INSERT should be [`None`] so DB can auto-increment it
    pub id: Option<i64>,
    /// ID of the region where telegram was transmitted
    pub region: i64,
    /// Reporting Point inside the r09 telegram (*meldepunkt*) ID
    pub reporting_point: i32,
    /// Report location latitude
    pub lat: f64,
    /// Report location longitude
    pub lon: f64,
    /// If this transmission postion inserted from absolute data, and all the inference for it
    /// should be ignored
    pub ground_truth: bool,
}

/// This struct queries the database for transmission locations inferred from every single trekkie
/// run. This is useful if you want to refine the position of [`TransmissionLocation`]
#[derive(Debug, Clone, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = r09_transmission_locations_raw)]
pub struct TransmissionLocationRaw {
    /// Primary key
    pub id: i64,
    /// ID of the region where telegram was transmitted
    pub region: i64,
    /// Reporting Point inside the r09 telegram (*meldepunkt*) ID
    pub reporting_point: i32,
    /// Report location latitude
    pub lat: f64,
    /// Report location longitude
    pub lon: f64,
    /// Trekkie run from which this undeduped location was inferred
    pub trekkie_run: uuid::Uuid,
    /// User, from whose trekkie run this undeduped location was inferred
    pub run_owner: uuid::Uuid,
}

/// This struct inserts into the table corresponding to [`TransmissionLocationRaw`]
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = r09_transmission_locations_raw)]
pub struct InsertTransmissionLocationRaw {
    /// Primary key. During INSERT should be [`None`] so DB can auto-increment it
    pub id: Option<i64>,
    /// ID of the region where telegram was transmitted
    pub region: i64,
    /// Reporting Point inside the r09 telegram (*meldepunkt*) ID
    pub reporting_point: i32,
    /// Report location latitude
    pub lat: f64,
    /// Report location longitude
    pub lon: f64,
    /// Trekkie run from which this undeduped location was inferred
    pub trekkie_run: uuid::Uuid,
    /// User, from whose trekkie run this undeduped location was inferred
    pub run_owner: uuid::Uuid,
}

/// Error for associated functions and methods over [`TransmissionLocation`] struct
pub enum TransmissionLocaionError {
    /// Input provided was empty
    EmptyInput,
    /// Expected the data to be from the same region, got from several instead.
    RegionMismatch,
}

type TransmissionLocationResult = Result<InsertTransmissionLocation, TransmissionLocaionError>;
impl TransmissionLocation {
    /// Maximum distance at which the raw point is considered to be corresponding to the report
    /// location cluster
    // TODO: Should we make this configurable?
    pub const MAX_SANE_DISTANCE: f64 = 50_f64;

    fn filter_outliers(input: Vec<TransmissionLocationRaw>) -> Option<Vec<TransmissionLocationRaw>> {
        // Edge case
        if input.len() == 0 {
            return None;
        }

        let (lats, lons): (Vec<f64>, Vec<f64>) = input.iter().map(|s| (s.lat, s.lon)).unzip();

        let (avg_lat, avg_lon) = (lats.iter().sum()/lats.len(), lons.iter().sum()/lons.len());

        let filtered: Vec<TransmissionLocationRaw> = Vec::new();

        for i in input {
            
        }

        Some(filtered)
    }

    /// This function creates the [`InsertTransmissionLocation`] from the vector of raw
    /// transmission locations. Points are averaged, any outliers further away then
    /// [`MAX_SANE_DISTANCE`] are discarded. All the [`TransmissionLocationRaw`] should have the
    /// same region, or the function will fail.
    ///
    /// **This is default way for updating the [`TransmissionLocation`]**. The analysis should be
    /// performed on the whole set of raw locations, to prevent biasing the data.
    pub fn new(raw: Vec<TransmissionLocationRaw>) -> TransmissionLocationResult {
        if raw.len() == 0 {
            return Err(TransmissionLocaionError::EmptyInput);
        }
        //let new_location = ;
        let region = raw[0].region;
        for loc in raw {
            if loc.region != region {
                return Err(TransmissionLocaionError::RegionMismatch);
            }
        }
        Err(TransmissionLocaionError::EmptyInput)
    }

    /// This function refines the existing [`TransmissionLocation`] with new data. Should be used
    /// with care, since by calling it on already processed runs would bias the data.
    fn refine(&mut self, raw: Vec<TransmissionLocationRaw>) -> TransmissionLocationResult {
        todo!("not implemented yet!")
    }
}

/// This trait calculates distance between two objects containing positional (latitude, longitude)
/// data.
pub trait DistanceFrom<T> {
    /// This function returns distance in meters between two objects
    fn distance_from(&self, value: T) -> f64;
}
