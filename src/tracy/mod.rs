use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::tracy_runs;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tracy_runs"]
pub struct TracyRun {
    pub id: i64,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub line: Option<i32>,
    pub run: Option<i32>,
    pub gps_file: Option<String>,
    pub region: i64,
    pub owner: Uuid,
    pub finished: bool
}

