use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

mod cpu_max;
use cpu_max::CPUMax;

mod cpu_weight;
use cpu_weight::CPUWeight;

mod memory_max;
use memory_max::MemoryMax;

mod memory_swap_max;
use memory_swap_max::MemorySwapMax;

mod cgroup_error;
use cgroup_error::CgroupError;

#[derive(Debug)]
pub struct Cgroup {
    path: PathBuf,
    cpu_max: CPUMax,
    cpu_weight: CPUWeight,
    memory_max: MemoryMax,
    memory_swap_max: MemorySwapMax,
}

pub type Result<T> = std::result::Result<T, CgroupError>;

impl Cgroup {
    /// Creates a new Cgroup with the specified name.
    /// `name`: The name for the new cgroup.
    /// `cpu_max_quota`: CPU time (μs) in `period`. `-1` for no limit.
    /// `cpu_max_period`: Period length (μs).
    /// `cpu_weight`: weight (1-10000) for allocating CPU time under contention.
    /// `memory_max`: Memory limit in bytes.
    /// `memory_swap_max`: Swap memory limit in bytes.
    pub fn build(
        name: &str,
        cpu_max_quota: Option<i64>,
        cpu_max_period: Option<u64>,
        cpu_weight: Option<u64>,
        memory_max: Option<u64>,
        memory_swap_max: Option<u64>,
    ) -> Result<Self> {
        let path = PathBuf::from(format!("/sys/fs/cgroup/{name}"));

        let cpu_max = match (cpu_max_quota, cpu_max_period) {
            (Some(q), Some(p)) => CPUMax::new(q, p),
            _ => CPUMax::default(),
        };

        let cpu_weight = match cpu_weight {
            Some(w) => CPUWeight::build(w)?,
            None => CPUWeight::default(),
        };

        let memory_max = match memory_max {
            Some(u) => MemoryMax::new(Some(u)),
            None => MemoryMax::default(),
        };

        let memory_swap_max = match memory_swap_max {
            Some(u) => MemorySwapMax::new(Some(u)),
            None => MemorySwapMax::default(),
        };

        Ok(Self {
            path,
            cpu_max,
            cpu_weight,
            memory_max,
            memory_swap_max,
        })
    }

    pub fn build_default(name: &str) -> Result<Self> {
        Cgroup::build(name, None, None, None, None, None)
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
    pub fn set_memory_max(&self, memory_max: u64) -> io::Result<()> {
        fs::write(self.path.join("memory.max"), memory_max.to_string())?;
        Ok(())
    }

    /// Sets swap memory limit in bytes.
    pub fn set_memory_swap_max(&self, memory_swap_max: u64) -> io::Result<()> {
        fs::write(
            self.path.join("memory.swap.max"),
            memory_swap_max.to_string(),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, num::NonZeroI128};

    #[test]
    fn test_cgroup_create_and_delete() {
        let test_cgroup_name = "test_cgroup";
        let cg = Cgroup::build_default(test_cgroup_name).unwrap();

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
