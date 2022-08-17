use crate::telegrams::r09::{R09SaveTelegram};

use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Clone)]
struct MeasurementInterval {
    start: Option<NaiveDateTime>,
    stop: Option<NaiveDateTime>,
    line: Option<i32>,
    run: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone)]
struct FinishedMeasurementInterval {
    start: NaiveDateTime,
    stop: NaiveDateTime,
    line: i32,
    run: i32,
}

impl FinishedMeasurementInterval {
    pub fn from_measurement(measurement: MeasurementInterval) -> FinishedMeasurementInterval { 
        FinishedMeasurementInterval {
            start: measurement.start.unwrap(),
            stop: measurement.stop.unwrap(),
            line: measurement.line.unwrap(),
            run: measurement.run.unwrap()
        }
    }

    pub fn fits(&self, telegram: &R09SaveTelegram) -> bool {
        assert!(telegram.line.is_none() || telegram.run_number.is_none(), "Cannot operate on data that doesn't have lines or runs");
        self.start < telegram.time
            && telegram.time < self.stop
            && telegram.line.unwrap() == self.line
            && telegram.run_number.unwrap() == self.run
    }
}
