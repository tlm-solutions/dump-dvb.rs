use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LineSegment {
    pub historical_time: u32, // time in seconds
    pub next_reporting_point: i32, // reporting_point
    pub positions: Vec<(f64, f64)>
}

/// departure reporting_point -> destiation reporting_point -> Vec<GPS>
pub type RegionGraph = HashMap<i32, Vec<LineSegment>>;

/// region_id -> region graph positions
pub type PositionGraph = HashMap<i32, RegionGraph>;
