use prometheus::HistogramVec;
use std::time::Instant;

pub struct Timer {
    histogram: HistogramVec,
    start: Instant,
    tags: &'static [&'static str],
}

impl Timer {
    pub fn new(histogram: HistogramVec, tags: &'static [&'static str]) -> Self {
        Self {
            histogram,
            start: Instant::now(),
            tags,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        self.histogram
            .with_label_values(self.tags)
            .observe(elapsed.as_secs_f64());
    }
}
