use std::error::Error;
use std::fmt;

use crate::cgroup::cpu_weight::CPUWeightError;
use crate::cgroup::io_max::IOMaxError;

#[derive(Debug)]
pub enum CgroupError {
    InvalidCPUConfiguration(CPUWeightError),
    InvalidIOConfiguration(IOMaxError),
}

impl fmt::Display for CgroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CgroupError::InvalidCPUConfiguration(err) => {
                write!(f, "Invalid CPU configuration: {}", err)
            }
            CgroupError::InvalidIOConfiguration(err) => {
                write!(f, "Invalid I/O configuration: {}", err)
            }
        }
    }
}

impl Error for CgroupError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CgroupError::InvalidCPUConfiguration(err) => Some(err),
            CgroupError::InvalidIOConfiguration(err) => Some(err),
        }
    }
}

impl From<CPUWeightError> for CgroupError {
    fn from(error: CPUWeightError) -> Self {
        CgroupError::InvalidCPUConfiguration(error)
    }
}

impl From<IOMaxError> for CgroupError {
    fn from(error: IOMaxError) -> Self {
        CgroupError::InvalidIOConfiguration(error)
    }
}
