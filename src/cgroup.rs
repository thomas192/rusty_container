use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Cgroup {
    path: PathBuf,
}

impl Cgroup {
    pub fn new(name: &str) -> Self {
        let path = PathBuf::from(format!("/sys/fs/cgroup/{name}"));
        Cgroup { path }
    }

    pub fn create(&self) -> io::Result<()> {
        fs::create_dir_all(&self.path)?;
        Ok(())
    }

    pub fn delete(&self) -> io::Result<()> {
        fs::remove_dir_all(&self.path)?;
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
