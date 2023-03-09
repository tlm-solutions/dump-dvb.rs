use crate::telegrams::r09::R09SaveTelegram;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Unfinished measurement of a vehicle. Mostly used by **trekkie** or **lofi** for correlation.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MeasurementInterval {
    /// Time the vehicle was entered.
    pub start: Option<NaiveDateTime>,
    /// Time the vehicle was left.
    pub stop: Option<NaiveDateTime>,
    /// Line (ger. linie) of the tracked vehicle.
    pub line: Option<i32>,
    /// Run (ger. kurs nummer) of the tracked vehicle.
    pub run: Option<i32>,
    /// Integer representing the region in which the vehicle was tracked.
    pub region: Option<i64>,
}

/// The FinishedMeasurementInterval struct is primarily used in **Wartrammer-40k** and **lofi**. It
/// defines the time interval and which vehicle was taken while data is actively being recorded.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FinishedMeasurementInterval {
    /// Time the the vehicle was entered.
    pub start: NaiveDateTime,
    /// Time the vehicle was left.
    pub stop: NaiveDateTime,
    /// Line (ger. linie) of the tracked vehicle.
    pub line: i32,
    /// Run (ger. kurs nummer) of the tracked vehicle.
    pub run: i32,
    /// Integer representing the region in which the vehicle was tracked.
    pub region: i64,
}

impl FinishedMeasurementInterval {
    /// Converts the intermediate representation into the final measurement.
    pub fn from_measurement(measurement: MeasurementInterval) -> FinishedMeasurementInterval {
        FinishedMeasurementInterval {
            start: measurement.start.unwrap(),
            stop: measurement.stop.unwrap(),
            line: measurement.line.unwrap(),
            run: measurement.run.unwrap(),
            region: measurement.region.unwrap(),
        }
    }

    /// Checks if a given Telegram corresponds to the measurement interval.
    pub fn fits(&self, telegram: &R09SaveTelegram) -> bool {
        if telegram.line.is_none() || telegram.run_number.is_none() {
            return false;
        }

        self.start < telegram.time
            && telegram.time < self.stop
            && telegram.line.unwrap() == self.line
            && telegram.run_number.unwrap() == self.run
            && telegram.region == self.region
    }
}
