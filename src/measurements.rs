use crate::telegrams::r09::{R09SaveTelegram};

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Clone)]
pub struct MeasurementInterval {
    pub start: Option<NaiveDateTime>,
    pub stop: Option<NaiveDateTime>,
    pub line: Option<i32>,
    pub run: Option<i32>,
    pub region: Option<i32>
}

/// The FinishedMeasurementInterval struct is primarly used in **Wartrammer-40k** and **lofi**.
/// It defines the time intervall and which vehicle was taken where data is actively being 
/// recorded.
#[derive(Deserialize, Serialize, Clone)]
pub struct FinishedMeasurementInterval {
    pub start: NaiveDateTime,
    pub stop: NaiveDateTime,
    pub line: i32,
    pub run: i32,
    pub region: i32,
}

impl FinishedMeasurementInterval {
    // Converts the intermediate representation into the final measurement
    pub fn from_measurement(measurement: MeasurementInterval) -> FinishedMeasurementInterval { 
        FinishedMeasurementInterval {
            start: measurement.start.unwrap(),
            stop: measurement.stop.unwrap(),
            line: measurement.line.unwrap(),
            run: measurement.run.unwrap(),
            region: measurement.region.unwrap()
        }
    }

    // Checks if a given Telegram was recorded in this time intervall
    pub fn fits(&self, telegram: &R09SaveTelegram) -> bool {
        if telegram.line.is_none() || telegram.run_number.is_none() {
            return false;
        }

        self.start < telegram.time
            && telegram.time < self.stop
            && telegram.line.unwrap() == self.line
            && telegram.run_number.unwrap() == self.run
    }
}



