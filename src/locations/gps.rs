use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use gpx::Gpx;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// ID value for [`GpsPoint`]'s that are not fetched from database, and therefore do not have a
/// primary key
pub const NO_ID: i64 = -0xDEADBABE;

/// Gps trackpoint representation used in database for Gps data
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = gps_points)]
pub struct GpsPoint {
    /// primary key bigserial
    pub id: i64,
    // foreign key referencing the corresponding trekkie run
    pub trekkie_run: Uuid,
    /// point's timestamp
    pub timestamp: NaiveDateTime,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Optional elevation
    pub elevation: Option<f64>,
    /// Optional GPS accuracy
    pub accuracy: Option<f64>,
    /// Optional GPS vertical accuracy
    pub vertical_accuracy: Option<f64>,
    /// Optional GPS bearing (aka angle of the compas)
    pub bearing: Option<f64>,
    /// Optional speed
    pub speed: Option<f64>,
}

/// struct for inserting a gps point into the database and utilise
/// the auto increment functions from postgres
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = gps_points)]
pub struct InsertGpsPoint {
    /// primary key bigserial
    pub id: Option<i64>,
    // foreign key referencing the corresponding trekkie run
    pub trekkie_run: Uuid,
    /// point's timestamp
    pub timestamp: NaiveDateTime,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Optional elevation
    pub elevation: Option<f64>,
    /// Optional GPS accuracy
    pub accuracy: Option<f64>,
    /// Optional GPS vertical accuracy
    pub vertical_accuracy: Option<f64>,
    /// Optional GPS bearing (aka angle of the compas)
    pub bearing: Option<f64>,
    /// Optional speed
    pub speed: Option<f64>,
}

/// Hasmap of gps unix timestamps to [`GpsPoint`]
#[derive(Clone, Debug)]
pub struct Gps(HashMap<i64, InsertGpsPoint>);


impl Gps {
    /// Extracts waypoints from all tracks and segments of a Gpx file
    pub fn insert_from_gpx_file(&mut self, filepath: &str) {
        let file = File::open(filepath).expect("Could not open gpx file.");
        let reader = BufReader::new(file);
        let gpx = gpx::read(reader).expect("could not parse gpx");
        Gps::insert_from_gpx(self, gpx)
    }

    // GPX WayPoint & soul extractor
    /// Gets Gpx type object and extracts all Waypoints from it, Returns gps::Gps.
    fn insert_from_gpx(&mut self, gpx: Gpx) {
        // I feel like my IQ dropping around here, but dunno how to do it, especially given time
        // situation in gpx crate
        for track in gpx.tracks {
            for segment in track.segments {
                for point in segment.points {
                    let soul = InsertGpsPoint {
                        id: None,
                        trekkie_run: Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
                        lat: point.point().y(), // according to gpx crate team x and y are less
                        lon: point.point().x(), // ambiguous for coordinates on a map
                        elevation: point.elevation,
                        timestamp: match point.time {
                            Some(time) => chrono::naive::NaiveDateTime::parse_from_str(
                                &time.format().unwrap(),
                                "%Y-%m-%dT%H:%M:%SZ",
                            )
                            .unwrap(),
                            None => break,
                        },

                        accuracy: point.pdop,
                        vertical_accuracy: point.vdop,
                        bearing: None,
                        speed: point.speed,
                    };

                    self.insert(soul.timestamp.timestamp(), soul);
                }
            }
        }
    }

    // hashmap boilerplate
    /// Exposes hashmap methods on our type alias
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = (&i64, &InsertGpsPoint)> {
        self.0.iter()
    }

    /// Exposes hashmap methods on our type alias
    #[allow(dead_code)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&i64, &mut InsertGpsPoint)> {
        self.0.iter_mut()
    }

    /// Exposes hashmap methods on our type alias
    pub fn insert(&mut self, k: i64, v: InsertGpsPoint) -> Option<InsertGpsPoint> {
        self.0.insert(k, v)
    }

    /// Exposes hashmap methods on our type alias
    pub fn get(&self, k: &i64) -> Option<&InsertGpsPoint> {
        self.0.get(k)
    }

    /// Exposes hashmap methods on our type alias
    pub fn empty() -> Gps {
        Gps(HashMap::new())
    }
}
