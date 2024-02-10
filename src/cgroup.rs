use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Cgroup {
    path: PathBuf,
}

impl Cgroup {
    /// Creates a new Cgroup with the specified name.
    /// `name`: The name for the new cgroup.
    pub fn new(name: &str) -> Self {
        let path = PathBuf::from(format!("/sys/fs/cgroup/{name}"));
        Cgroup { path }
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
    /// `weight`: weight (1-10000) for allocating CPU time under contention.
    pub fn set_cpu_weight(&self, weight: u64) -> io::Result<()> {
        if weight < 1 || weight > 10000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "CPU weight must be between 1 and 10000.",
            ));
        }
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
        let cg = Cgroup::new(test_cgroup_name);

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
