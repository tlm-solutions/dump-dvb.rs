use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Struct saving locations in Epsg3857 format which is used by openstreet map.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Epsg3857 {
    /// position on the x - axis
    pub x: f32,
    /// position on the y - axis
    pub y: f32,
}

/// Struct saving a position with extra properties, which could contain extra information about
/// this location.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Position {
    /// latitude of the position
    pub lat: f32,
    /// longitude of the position
    pub lon: f32,
    /// extra properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// This Data Structure is a intermediate representation used in lofi for generating the graph
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Edge {
    /// when last updated.
    pub last_update: u64,
    /// historical time that was required to traverse this edge.
    pub historical_time: u64,
    /// next expected reporting point when following the edge.
    pub next_reporting_point: u32,
    /// hashmap of positions along the track the string encodes the percentage of the path
    /// traveled. So possible value maybe 33 -> Position this would mean after 33% of the time 
    /// the vehicle is expected to be at this position.
    pub positions: HashMap<String, Position>,
}

/// This struct encodes position between reporting points with relative times so they can be used
/// to interpolate the position of the vehilce between reporting_points.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LineSegment {
    /// historical time in milli-seconds that was required to traverse this edge.
    pub historical_time: u32,
    /// next expected reporting point when following the edge.
    pub next_reporting_point: i32,
    /// hashmap of positions along the track the string encodes the percentage of the path
    /// traveled. So possible value maybe 33 -> Position this would mean after 33% of the time 
    /// the vehicle is expected to be at this position.
    pub positions: HashMap<String, Position>,
}

/// departure reporting_point -> destiation reporting_point -> Vec<GPS>
pub type RegionGraph = HashMap<i32, Vec<LineSegment>>;

/// region_id -> region graph positions
pub type PositionGraph = HashMap<i64, RegionGraph>;
