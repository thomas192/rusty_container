#[derive(Debug)]
pub struct CPUMax {
    quota: i64,
    period: u64,
}

impl CPUMax {
    pub fn new(quota: i64, period: u64) -> Self {
        CPUMax { quota, period }
    }

    pub fn quota(&self) -> i64 {
        self.quota
    }

    pub fn period(&self) -> u64 {
        self.period
    }
}

impl Default for CPUMax {
    fn default() -> Self {
        CPUMax {
            quota: -1,
            period: 100_000, // does not matter when `quota` is set to -1
        }
    }
}
