use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Position {
    lat: f32,
    lon: f32,
    properties: HashMap<String, serde_json::Value> 
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Edge {
    last_update: u64,
    historical_time: u64,
    next_reporting_point: u32,
    positions: HashMap<String, Position>
}


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
