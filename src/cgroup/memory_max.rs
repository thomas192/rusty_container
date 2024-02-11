#[derive(Debug)]
pub struct MemoryMax(Option<u64>);

impl MemoryMax {
    pub fn new(max: Option<u64>) -> Self {
        Self(max)
    }

    pub fn value(&self) -> Option<u64> {
        self.0
    }
}

impl Default for MemoryMax {
    fn default() -> Self {
        Self(
            // No limit by default, representing the default OS behavior for a new cgroup
            None,
        )
    }
}
