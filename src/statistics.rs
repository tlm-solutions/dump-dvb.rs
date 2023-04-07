use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::{region_statistics, station_statistics, user_statistics};

/// Statistics for Regions
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = region_statistics)]
pub struct RegionStatistics {
    /// region identifier
    pub id: i64,
    /// timestamp when the last update happend
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub last_updated: NaiveDateTime,
    /// total amount of telegrams received in this region
    pub total_telegrams: i64,
    /// amount of telegrams received in this region in the last month
    pub month_telegrams: i64,
    /// amount of telegrams received in this region in the last week
    pub week_telegrams: i64,
    /// amount of telegrams received in this region in the last 24h
    pub day_telegrams: i64,
    /// total amount of gps points received in this region
    pub total_gps: i64,
    /// amount of gps points received in this region in the last month
    pub month_gps: i64,
    /// amount of gps points received in this region in the last week
    pub week_gps: i64,
    /// amount of gps points received in this region in the last 24h
    pub day_gps: i64,
}

/// Statistics for Stations
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = station_statistics)]
pub struct StationStatistics {
    /// station identifier
    pub id: Uuid,
    /// timestamp when the last update happend
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub last_updated: NaiveDateTime,
    /// total amount of telegrams received in this region
    pub total_telegrams: i64,
    /// amount of telegrams received in this region in the last month
    pub month_telegrams: i64,
    /// amount of telegrams received in this region in the last week
    pub week_telegrams: i64,
    /// amount of telegrams received in this region in the last 24h
    pub day_telegrams: i64,
}

/// Statistics for Users
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = user_statistics)]
pub struct UserStatistics {
    /// user identifier
    pub id: Uuid,
    /// timestamp when the last update happend
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub last_updated: NaiveDateTime,
    /// total amount of gps points received in this region
    pub total_gps: i64,
    /// amount of gps points received in this region in the last month
    pub month_gps: i64,
    /// amount of gps points received in this region in the last week
    pub week_gps: i64,
    /// amount of gps points received in this region in the last 24h
    pub day_gps: i64,
}
