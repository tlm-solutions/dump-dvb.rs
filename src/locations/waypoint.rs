//! This module defines structs and enum working with waypoints

/// this enum tell the waypoint which source they came from
pub enum WayPointType {
    /// position deduced from received r09 telegram
    R09Telegram = 0,
    /// position submitted via trekkie service
    TrekkieGPS = 1
}

