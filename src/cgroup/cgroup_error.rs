use std::error::Error;
use std::fmt;

use crate::cgroup::cpu_weight::CPUWeightError;

#[derive(Debug)]
pub enum CgroupError {
    InvalidConfiguration(CPUWeightError),
}

impl fmt::Display for CgroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CgroupError::InvalidConfiguration(err) => write!(f, "Invalid configuration: {}", err),
        }
    }
}

impl Error for CgroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CgroupError::InvalidConfiguration(err) => Some(err),
        }
    }
}

impl From<CPUWeightError> for CgroupError {
    fn from(error: CPUWeightError) -> Self {
        CgroupError::InvalidConfiguration(error)
    }
}
