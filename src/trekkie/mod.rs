use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::trekkie_runs;

/// This saves a measurement of a vehicle for later correlation. This struct tells when the vehicle
/// was measured and what are line and run number. The gps_table references trekkie_runs for all
/// the saved gps locations
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = trekkie_runs)]
pub struct TrekkieRun {
    /// time when the vehicle was entered
    pub start_time: NaiveDateTime,
    /// time when the vehicle was left
    pub end_time: NaiveDateTime,
    /// line (ger. linie) of the vehicle
    pub line: i32,
    /// run (ger. kurs) of the vehicle
    pub run: i32,
    /// integer representing the integer
    pub region: i64,
    /// UUID of the user that made the trekkie run
    pub owner: Uuid,
    /// flag that tell if the trekkie run is finished or still in progress
    pub finished: bool,
    /// primary key to uniquely identify a trekkie run
    pub id: Uuid,
    /// flag that tells that the data recorded for this trekkie run is already correlated
    pub correlated: bool,
}
