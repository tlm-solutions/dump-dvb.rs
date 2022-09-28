use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::trekkie_runs;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = trekkie_runs)]
pub struct TrekkieRun {
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
