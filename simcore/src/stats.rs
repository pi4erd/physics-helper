use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TimedData<T> {
    time: f64, // offset from start in seconds
    data: T,
}

pub struct Timeseries<T> {
    start_time: Instant,
    series: Vec<TimedData<T>>,
}

impl<T> Timeseries<T> {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            series: Vec::new(),
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()>
        where T: Serialize
    {
        std::fs::write(filename, serde_json::to_string(&self.series)?)
    }

    pub fn record(&mut self, data: T) {
        self.series.push(TimedData {
            time: (Instant::now() - self.start_time).as_secs_f64(),
            data,
        });
    }
}
