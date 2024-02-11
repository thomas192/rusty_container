use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CPUWeightError {
    OutOfRange,
}

impl fmt::Display for CPUWeightError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CPUWeightError::OutOfRange => write!(f, "CPU weight must be between 1 and 10000."),
        }
    }
}

impl Error for CPUWeightError {}

impl Default for CPUWeight {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Debug)]
pub struct CPUWeight(u64);

pub type Result<T> = std::result::Result<T, CPUWeightError>;

impl CPUWeight {
    pub fn build(weight: u64) -> Result<Self> {
        if weight < 1 || weight > 10000 {
            Err(CPUWeightError::OutOfRange)
        } else {
            Ok(Self(weight))
        }
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpuweight_build() {
        let res = CPUWeight::build(20000);
        assert!(matches!(res, Err(CPUWeightError::OutOfRange)));

        let res = CPUWeight::build(1000);
        assert_eq!(res.unwrap().value(), 1000);

        let default_weight = CPUWeight::default();
        assert_eq!(default_weight.value(), 100);
    }
}
