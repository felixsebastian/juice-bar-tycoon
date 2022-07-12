use std::time::{Duration, Instant};

pub struct ProgressTimer {
    time: Instant,
    duration: Duration
}

impl ProgressTimer {
    pub fn start(seconds: u64) -> ProgressTimer {
        ProgressTimer {
            time: Instant::now(),
            duration: Duration::new(seconds, 0)
        }
    }

    pub fn decimal(& self) -> f64 {
        self.time.elapsed().as_millis() as f64 / self.duration.as_millis() as f64
    }

    pub fn done(& self) -> bool {
        self.decimal() > 1.
    }
}

impl std::fmt::Display for ProgressTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.0}%",  self.decimal() * 100.)
    }
}
