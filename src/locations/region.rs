//! This module holds structs and associated stuff for storing the region metadata ([`Region`] and
//! [`InsertRegion`], as well as some region-related structs that are employed for caching the
//! region data.

use crate::schema::*;
use crate::telegrams::r09::R09Type;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Default region cache lifetime in seconds (24h)
pub const REGION_CACHE_EXPIRATION: i64 = 24 * 60 * 60;
/// Name for a cache file
pub const REGION_CACHE_FILE: &'static str = "region_cache.json";
/// maximum distance in meters
pub const SANE_INTERPOLATION_DISTANCE: i32 = 50;

/// Struct holding the information for a region.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Region {
    /// Unique region identifier this is really just an arbitrery number.
    #[diesel(deserialize_as = i64)]
    pub id: i64,
    /// Name of the region / city
    pub name: String,
    /// Name of the operator in the city e.g DVB.
    pub transport_company: String,
    /// Name of the Regional operator e.g. VVO (Verkehrs Verbund Oberelbe)
    /// which encompasses the transport_companty
    pub regional_company: Option<String>,
    /// The frequency the operator sends it VDV 420 traffic.
    pub frequency: Option<i64>,
    /// Which R09 types are used look at [`R09Type`][crate::telegrams::r09::R09Type] for possible
    /// values
    pub r09_type: Option<R09Type>,
    /// Which encoding this regions uses. Look at [`Encoding`] for possible values.
    pub encoding: Option<i32>,
    /// This value is set to true if the region is deleted.
    pub deactivated: bool,
}

/// This struct is the same as [`Region`] but with the difference that id is optional
/// this is required to use the auto increment function from postgres
#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = regions)]
pub struct InsertRegion {
    /// Unqiue region identifier which is nullable to let postgres set a value for us.
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    /// Name of the region / city
    pub name: String,
    /// Name of the operator in the city e.g DVB.
    pub transport_company: String,
    /// Name of the Regional operator e.g. VVO (Verkehrs Verbund Oberelbe)
    /// which encompasses the transport_companty
    pub regional_company: Option<String>,
    /// The frequency the operator sends it VDV 420 traffic.
    pub frequency: Option<i64>,
    /// Which R09 types are used look at [`R09Type`][crate::telegrams::r09::R09Type] for possible
    /// values
    pub r09_type: Option<R09Type>,
    /// Which encoding this regions used look at [`Encoding`] for possible values.
    pub encoding: Option<i32>,
    /// This value is set to true if the region is deleted.
    pub deactivated: bool,
}

/// The struct that deserializes into json containing cache for region meta information. See
/// [`Region`] for details.
#[derive(Serialize, Deserialize, Debug)]
pub struct RegionCache {
    /// Hashmap of region ID to corresponding metadata
    pub metadata: HashMap<i64, Region>,
    /// Timestamp when the cache was last refreshed
    pub modified: DateTime<Utc>,
}

/// Error enum for [`LocationsJson`] methods and associated funcitons. Right now it just returns
/// original error wrapped into an enum variant.
#[derive(Debug)]
pub enum RegionCacheError {
    /// See [`serde_json::Error`]
    SerdeJsonError(serde_json::Error),
    /// See [`reqwest::Error`]
    ReqwestError(reqwest::Error),
    /// See [`std::io::Error`]
    IOError(std::io::Error),
}

impl From<reqwest::Error> for RegionCacheError {
    fn from(e: reqwest::Error) -> RegionCacheError {
        RegionCacheError::ReqwestError(e)
    }
}
impl From<serde_json::Error> for RegionCacheError {
    fn from(e: serde_json::Error) -> RegionCacheError {
        RegionCacheError::SerdeJsonError(e)
    }
}
impl From<std::io::Error> for RegionCacheError {
    fn from(e: std::io::Error) -> RegionCacheError {
        RegionCacheError::IOError(e)
    }
}

impl RegionCache {
    const REGION_API_ENDPOINT: &'static str = "/region";
    /// refreshes the region data cache from the datacare API unconditionaly.
    pub fn get_region_cache(
        datacare_api: &str,
        cache_dir: PathBuf,
    ) -> Result<Self, RegionCacheError> {
        let api_url = format!(
            "{datacare_api}{endpoint}",
            endpoint = Self::REGION_API_ENDPOINT
        );
        let api_response: String = reqwest::blocking::get(api_url)?.text()?;

        let region_cache: HashMap<i64, Region> = serde_json::from_str(&api_response)?;

        let timestamped_region_cache = Self {
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
    pub fn read_region_cache(cache_dir: PathBuf) -> Result<Self, RegionCacheError> {
        let mut cache_file_path = cache_dir;
        cache_file_path.push(REGION_CACHE_FILE);
        let cache_file_string = fs::read_to_string(cache_file_path)?;

        let cache = serde_json::from_str::<Self>(&cache_file_string)?;
        Ok(cache)
    }

    /// Gets the cache for the region data. First looks if it exists already, if not (or if it is
    /// older than [`REGION_CACHE_EXPIRATION`]) tries to update it. Any Errs are propagated up.
    pub fn update_region_cache(
        datacare_api: &str,
        cache_dir: PathBuf,
    ) -> Result<Self, RegionCacheError> {
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
}
