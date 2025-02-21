use meca::stats::Stats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VueStats {
    pub count: u64,
    pub mean: f64,
    pub variance: f64,
}

impl VueStats {
    /// Create a new VueStats from a Stats object
    pub fn from_stats(stats: &Stats) -> Self {
        VueStats {
            count: stats.count(),
            mean: stats.mean(),
            variance: stats.variance(),
        }
    }
}
