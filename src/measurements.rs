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
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub start: NaiveDateTime,
    /// Time the vehicle was left.
    #[serde(serialize_with = "crate::serialize_with_zone")]
    pub stop: NaiveDateTime,
    /// Line (ger. linie) of the tracked vehicle.
    pub line: i32,
    /// Run (ger. kurs nummer) of the tracked vehicle.
    pub run: i32,
    /// Integer representing the region in which the vehicle was tracked.
    pub region: i64,
}

#[allow(missing_docs)] // allowed since enum variants are pretty self-explanatory
/// This enum signifies error during conversion of [`MeasurementInterval`] into
/// [`FinishedMeasurementInterval`]. Variants are pretty self-explanatory
pub enum MeasuerementIntervalError {
    MissingStartValue,
    MissingStopValue,
    MissingLineValue,
    MissingRunValue,
    MissingRegionValue,
}

impl TryFrom<MeasurementInterval> for FinishedMeasurementInterval {
    type Error = MeasuerementIntervalError;
    fn try_from(val: MeasurementInterval) -> Result<Self, Self::Error> {
        // match every value, if any of them is None, return corresponding error
        let start = match val.start {
            Some(v) => v,
            None => {
                return Err(MeasuerementIntervalError::MissingStartValue);
            }
        };
        let stop = match val.stop {
            Some(v) => v,
            None => {
                return Err(MeasuerementIntervalError::MissingStopValue);
            }
        };
        let line = match val.line {
            Some(v) => v,
            None => {
                return Err(MeasuerementIntervalError::MissingLineValue);
            }
        };
        let run = match val.run {
            Some(v) => v,
            None => {
                return Err(MeasuerementIntervalError::MissingRunValue);
            }
        };
        let region = match val.region {
            Some(v) => v,
            None => {
                return Err(MeasuerementIntervalError::MissingRegionValue);
            }
        };

        Ok(FinishedMeasurementInterval {
            start,
            stop,
            line,
            run,
            region,
        })
    }
}

impl FinishedMeasurementInterval {
    #[deprecated(
        since = "0.8.1",
        note = "Please use TryFrom<MeasurementInterval> trait from now on!"
    )]
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
