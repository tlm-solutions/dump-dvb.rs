use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Epsg3857 {
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Position {
    pub lat: f32,
    pub lon: f32,
    pub properties: HashMap<String, serde_json::Value> 
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Edge {
    pub last_update: u64,
    pub historical_time: u64,
    pub next_reporting_point: u32,
    pub positions: HashMap<String, Position>
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LineSegment {
    pub historical_time: u32, // time in seconds
    pub next_reporting_point: i32, // reporting_point
    pub positions: HashMap<String, Position>
}

/// departure reporting_point -> destiation reporting_point -> Vec<GPS>
pub type RegionGraph = HashMap<i32, Vec<LineSegment>>;

/// region_id -> region graph positions
pub type PositionGraph = HashMap<i32, RegionGraph>;
