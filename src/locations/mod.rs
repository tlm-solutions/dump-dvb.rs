pub mod gps;
pub mod region;
mod tests;
pub mod waypoint;

use crate::schema::*;
use crate::telegrams::r09::R09Type;

use chrono::prelude::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

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
#[derive(Debug, Clone, Queryable)]
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
}

/// This struct queries the database for transmission locations inferred from every single trekkie
/// run. This is useful if you want to refine the position of [`TransmissionLocation`]
#[derive(Debug, Clone, Queryable)]
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

/// Meta inforamtion about region.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RegionMetaInformation {
    /// Frequency in Hz
    pub frequency: Option<u64>,
    /// Human-readable name of the region
    pub city_name: Option<String>,
    /// Type of R09 telegram used in the region
    pub type_r09: Option<R09Type>,
    /// Latitude of the Region, degrees
    pub lat: Option<f64>,
    /// Longitude of the Region, degrees
    pub lon: Option<f64>,
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

/// Doucment meta information. To set metadata for the document see [`StopsJson::construct`]
/// or to [`StopsJson::update_metadata`]
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
    /// Hash map of a region number to the [`RegionReportLocations`]
    pub data: HashMap<i64, RegionReportLocations>,
    /// Hash map of a region number to the meta information about this region
    pub meta: HashMap<i64, RegionMetaInformation>,
}

/// Merge statistics for [`LocationsJson`]
pub struct LocationsJsonMergeStats {
    /// Amount of new reporting locations added by merge
    pub created: u64,
    /// Amount of reporting locations that were refined by merge
    pub refined_loc: u64,
    /// Amount of reporting locations that were skipped, e.g. due to the failed sanity checks
    pub skipped: u64,
}

/// The struct that deserializes into json containing cache for region meta information. See
/// [`RegionMetaInformation`] for details.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegionMetaCache {
    /// Hashmap of region ID to corresponding metadata
    pub metadata: HashMap<i64, RegionMetaInformation>,
    /// Timestamp when the cache was last refreshed
    pub modified: DateTime<Utc>,
}

/// Error enum for [`LocationsJson`] methods and associated funcitons. Right now it just returns
/// original error wrapped into an enum variant.
#[derive(Debug)]
pub enum LocationsJsonError {
    /// See [`serde_json::Error`]
    SerdeJsonError(serde_json::Error),
    /// See [`reqwest::Error`]
    ReqwestError(reqwest::Error),
    /// See [`std::io::Error`]
    IOError(std::io::Error),
}

impl From<reqwest::Error> for LocationsJsonError {
    fn from(e: reqwest::Error) -> LocationsJsonError {
        LocationsJsonError::ReqwestError(e)
    }
}
impl From<serde_json::Error> for LocationsJsonError {
    fn from(e: serde_json::Error) -> LocationsJsonError {
        LocationsJsonError::SerdeJsonError(e)
    }
}
impl From<std::io::Error> for LocationsJsonError {
    fn from(e: std::io::Error) -> LocationsJsonError {
        LocationsJsonError::IOError(e)
    }
}

impl LocationsJson {
    /// Deserialzes file into [`LocationsJson`]
    pub fn from_file(file: &str) -> Result<LocationsJson, serde_json::error::Error> {
        let data = fs::read_to_string(file).expect("could not read LocationsJson file!");
        serde_json::from_str(&data)
    }

    /// Creates the [`LocationsJson`] struct form the hashmaps for data and meta fields, while taking
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

    /// Serialises [`LocationsJson`] to json file. If file exists - silently overwrites it.
    pub fn write(&self, file: &str) {
        fs::remove_file(file).ok();
        let mut output = File::create(file).expect("cannot create or open file!");

        let json_data = serde_json::to_string_pretty(&self).expect("cannot serialize structs!");

        output
            .write_all(json_data.as_bytes())
            .expect("cannot write to file!");
    }

    /// Populates document metainformation.
    pub fn update_metadata(
        &mut self,
        generator: Option<String>,
        generator_version: Option<String>,
    ) {
        self.document = DocumentMeta {
            schema_version: String::from(SCHEMA),
            date: chrono::Utc::now(),
            generator,
            generator_version,
        };
    }

    /// refreshes the region data cache from the datacare API unconditionaly.
    pub fn get_region_cache(
        datacare_api: &str,
        cache_dir: PathBuf,
    ) -> Result<RegionMetaCache, LocationsJsonError> {
        let api_url = format!("{datacare_api}/region");
        let api_response: String = reqwest::blocking::get(api_url)?.text()?;

        let region_cache: HashMap<i64, RegionMetaInformation> =
            serde_json::from_str(&api_response)?;

        let timestamped_region_cache = RegionMetaCache {
            metadata: region_cache,
            modified: Utc::now(),
        };

        // try to write out the cache
        let mut cache_file = cache_dir;
        cache_file.push(REGION_CACHE_FILE);
        let cache_string = serde_json::to_string(&timestamped_region_cache)?;
        fs::write(cache_file, cache_string)?;

        Ok(timestamped_region_cache)
    }

    /// Read region cache from local cache path.
    pub fn read_region_cache(cache_dir: PathBuf) -> Result<RegionMetaCache, LocationsJsonError> {
        let mut cache_file_path = cache_dir;
        cache_file_path.push(REGION_CACHE_FILE);
        let cache_file_string = fs::read_to_string(cache_file_path)?;

        let cache = serde_json::from_str::<RegionMetaCache>(&cache_file_string)?;
        Ok(cache)
    }

    /// Gets the cache for the region data. First looks if it exists already, if not (or if it is
    /// older than [`REGION_CACHE_EXPIRATION`]) tries to update it. Any Errs are propagated up.
    pub fn update_region_cache(
        datacare_api: &str,
        cache_dir: PathBuf,
    ) -> Result<RegionMetaCache, LocationsJsonError> {
        let mut cache_file_path = cache_dir.clone();
        cache_file_path.push(REGION_CACHE_FILE);

        // make sure that the dir exists
        fs::create_dir_all(cache_dir.clone())?;

        // try to read the cache
        let cache_to_return = match Self::read_region_cache(cache_dir.clone()) {
            Ok(read_cache) => {
                // check that cache is fresh enough
                if (Utc::now() - read_cache.modified)
                    < chrono::Duration::seconds(REGION_CACHE_EXPIRATION)
                {
                    read_cache
                } else {
                    // try to update the cache
                    match Self::get_region_cache(datacare_api, cache_dir) {
                        Ok(new_cache) => new_cache,
                        Err(e) => {
                            eprintln!("While trying to get the cache from datacare API: {e:?}");
                            eprintln!("Using stale cache! {read_cache:?}");
                            read_cache
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("While trying to get local region metadata cache: {e:?}");
                eprintln!("Trying to refresh region metadata cache");
                Self::get_region_cache(datacare_api, cache_dir)?
            }
        };

        Ok(cache_to_return)
    }

    /// Updates the region information in the file from the cache
    pub fn update_region_data(&mut self, region_cache: HashMap<i64, RegionMetaInformation>) {
        // iterate over keys in data, put it into hashset
        let mut reg_set: HashSet<i64> = HashSet::new();
        // iterate over hashset, put region metadata
        for k in self.data.keys() {
            reg_set.insert(*k);
        }

        for r in reg_set {
            match region_cache.get(&r) {
                Some(reg) => {
                    self.meta.insert(r, reg.clone());
                }
                None => {
                    eprintln!("ERROR: Region {r} is not found in region metadata cache!");
                    eprintln!("WARNING: Metadata for region {r} is not written!");
                }
            };
        }
    }

    /// This method merges in new report locations. Averages the transmission position in the
    /// process. Returns statistic ([`LocationsJsonMergeStats`] wrapped in [`Result`]): how many new report
    /// locations are added, how many transmission positions were refined, how many points were
    /// discarded.
    pub fn merge(
        &mut self,
        new: &LocationsJson,
        region_cache: HashMap<i64, RegionMetaInformation>,
    ) -> Result<LocationsJsonMergeStats, LocationsJsonError> {
        let mut new_regions: HashSet<i64> = HashSet::new();
        let mut stats = LocationsJsonMergeStats {
            created: 0,
            refined_loc: 0,
            skipped: 0,
        };
        for k in new.data.keys() {
            new_regions.insert(*k);
        }

        for r in new_regions {
            let region_data = self.data.entry(r).or_insert(HashMap::new());
            let new_region_data = new.data.get(&r).expect("Unreacheble");
            for (k, v) in new_region_data {
                region_data
                    .entry(*k)
                    .and_modify(|old| *old = Self::linear_interpolate_point(old, v, &mut stats))
                    .or_insert({
                        stats.created += 1;
                        v.clone()
                    });
            }
        }

        self.update_region_data(region_cache);

        Ok(stats)
    }

    /// Takes two GPS points, lineraly interpolates them while performing sanity checks.
    fn linear_interpolate_point(
        old: &ReportLocation,
        new: &ReportLocation,
        stats: &mut LocationsJsonMergeStats,
    ) -> ReportLocation {
        let a = (old.lat - new.lat).sin().powi(2)
            + new.lat.cos() * old.lat.cos() * (old.lat - new.lat).sin().powi(2);
        let c = 2_f64 * a.sqrt().atan2((1_f64 - a).sqrt());
        let distance = MEAN_EARTH_RADIUS as f64 * c;

        if distance > SANE_INTERPOLATION_DISTANCE.into() {
            stats.skipped += 1;
            return old.clone();
        }

        let lon = new.lon + old.lon / 2.;
        let lat = new.lat + old.lat / 2.;
        stats.refined_loc += 1;

        ReportLocation {
            lat,
            lon,
            properties: serde_json::Value::Null,
        }
    }
}
