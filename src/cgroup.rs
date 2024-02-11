use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

mod cpuweight;
use cpuweight::CPUWeight;

mod cgroup_error;
use cgroup_error::CgroupError;

#[derive(Debug)]
pub struct Cgroup {
    path: PathBuf,
    cpu_weight: CPUWeight,
}

pub type Result<T> = std::result::Result<T, CgroupError>;

impl Cgroup {
    /// Creates a new Cgroup with the specified name.
    /// `name`: The name for the new cgroup.
    /// `cpu_weight`: weight (1-10000) for allocating CPU time under contention.
    pub fn build(name: &str, cpu_weight: Option<u64>) -> Result<Self> {
        let path = PathBuf::from(format!("/sys/fs/cgroup/{name}"));

        let cpu_weight = match cpu_weight {
            Some(w) => CPUWeight::build(w)?,
            None => CPUWeight::default(),
        };

        Ok(Self { path, cpu_weight })
    }

    /// Creates the cgroup in the system.
    pub fn create(&self) -> io::Result<()> {
        fs::create_dir_all(&self.path)?;
        Ok(())
    }

    /// Deletes the cgroup from the system.
    pub fn delete(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.path)?;
        Ok(())
    }

    /// Sets CPU usage limit.
    /// `quota`: CPU time (μs) in `period`. `-1` for no limit.
    /// `period`: Period length (μs).
    pub fn set_cpu_max(&self, quota: i64, period: u64) -> io::Result<()> {
        let max_value = format!("{} {}", quota, period);
        fs::write(self.path.join("cpu.max"), max_value)?;
        Ok(())
    }

    /// Sets the CPU weight.
    pub fn set_cpu_weight(&self, weight: u64) -> io::Result<()> {
        fs::write(self.path.join("cpu.weight"), weight.to_string())?;
        Ok(())
    }

    /// Sets memory limit in bytes.
    /// `limit`: Memory limit in bytes.
    pub fn set_memory_limit(&self, limit: u64) -> io::Result<()> {
        fs::write(self.path.join("memory.max"), limit.to_string())?;
        Ok(())
    }

    /// Sets swap memory limit in bytes.
    /// `limit`: Swap limit in bytes.
    pub fn set_swap_limit(&self, limit: u64) -> io::Result<()> {
        fs::write(self.path.join("memory.swap.max"), limit.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_cgroup_create_and_delete() {
        let test_cgroup_name = "test_cgroup";
        let cg = Cgroup::build(test_cgroup_name, None).unwrap();

        assert!(cg.create().is_ok(), "Cgroup should be created successfully");

        assert!(
            fs::metadata(&cg.path).is_ok(),
            "Cgroup directory should exist after creation"
        );

        assert!(cg.delete().is_ok(), "Cgroup should be deleted successfully");

        assert!(
            fs::metadata(&cg.path).is_err(),
            "Cgroup directory should not exist after deletion"
        );
    }
}
