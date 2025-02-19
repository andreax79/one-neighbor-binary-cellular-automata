use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Stats {
    mean: f64,
    sum_squared_diffs: f64,
    count: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            mean: 0.0,
            sum_squared_diffs: 0.0,
            count: 0,
        }
    }

    /// Update mean and variance (Welford's online algorithm)
    pub fn update(&mut self, value: f64) {
        let delta = value - self.mean;
        self.count += 1;
        self.mean += delta / self.count as f64;
        self.sum_squared_diffs += delta * (value - self.mean);
    }

    /// Get the number of samples
    pub fn count(&self) -> u64 {
        self.count
    }

    /// Get the mean
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Get the unbiased variance
    pub fn variance(&self) -> f64 {
        if self.count > 1 {
            // sum((x - mean)^2) / (count - 1)
            self.sum_squared_diffs / (self.count - 1) as f64
        } else {
            std::f64::NAN
        }
    }
}

impl fmt::Display for Stats {
    /// Implement the Display trait for Stats to print the mean and variance
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mean: {:.6}\nVariance: {:.6}",
            self.mean(),
            self.variance()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stats = Stats::new();
        assert!(stats.mean() == 0.0);
        assert!(stats.variance().is_nan());
        assert!(stats.count() == 0);
    }

    #[test]
    fn test_update_single_value() {
        let mut stats = Stats::new();
        stats.update(10.0);

        assert!(stats.mean() == 10.0);
        assert!(stats.variance().is_nan());
        assert!(stats.count() == 1);
    }

    #[test]
    fn test_update_multiple_values() {
        let mut stats = Stats::new();
        stats.update(2.0);
        stats.update(4.0);
        stats.update(6.0);

        let expected_mean = 4.0;
        let expected_variance = 4.0;

        assert!((stats.mean() - expected_mean).abs() < 1e-6);
        assert!((stats.variance() - expected_variance).abs() < 1e-6);
        assert!(stats.count() == 3);
    }
}
