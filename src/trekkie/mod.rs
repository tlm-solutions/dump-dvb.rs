use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::trekkie_runs;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = trekkie_runs)]
pub struct TrekkieRun {
    pub id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub line: i32,
    pub run: i32,
    pub region: i64,
    pub owner: Uuid,
    pub finished: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = trekkie_runs)]
pub struct InsertTrekkieRun {
    pub id: Option<Uuid>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub line: i32,
    pub run: i32,
    pub region: i64,
    pub owner: Uuid,
    pub finished: bool,
}
