use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::trekkie_runs;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = trekkie_runs)]
pub struct TrekkieRun {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub line: i32,
    pub run: i32,
    pub region: i64,
    pub owner: Uuid,
    pub finished: bool,
    pub id: Uuid,
}

