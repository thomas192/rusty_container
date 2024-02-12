use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum IOMaxError {
    InvalidFormat,
    InvalidDevice,
    InvalidLimit,
}

impl fmt::Display for IOMaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IOMaxError::InvalidFormat => write!(f, "Invalid format. Expected '<device>:<limit>'."),
            IOMaxError::InvalidDevice => write!(f, "Invalid device specified."),
            IOMaxError::InvalidLimit => {
                write!(f, "Invalid limit. The limit must be a positive number.")
            }
        }
    }
}

impl Error for IOMaxError {}

#[derive(Debug, Default)]
pub struct IOMax {
    pub device_read_bps: Option<String>,
    pub device_write_bps: Option<String>,
    pub device_read_iops: Option<String>,
    pub device_write_iops: Option<String>,
}

pub type Result<T> = std::result::Result<T, IOMaxError>;

impl IOMax {
    pub fn build(
        device_read_bps: Option<String>,
        device_write_bps: Option<String>,
        device_read_iops: Option<String>,
        device_write_iops: Option<String>,
    ) -> Result<Self> {
        let device_read_bps = device_read_bps
            .as_deref()
            .map(Self::validate_and_transform)
            .transpose()?;
        let device_write_bps = device_write_bps
            .as_deref()
            .map(Self::validate_and_transform)
            .transpose()?;
        let device_read_iops = device_read_iops
            .as_deref()
            .map(Self::validate_and_transform)
            .transpose()?;
        let device_write_iops = device_write_iops
            .as_deref()
            .map(Self::validate_and_transform)
            .transpose()?;

        Ok(Self {
            device_read_bps,
            device_write_bps,
            device_read_iops,
            device_write_iops,
        })
    }

    // Converts a device path (e.g., /dev/sda) to a major:minor device number string.
    fn device_path_to_major_minor(device_path: &str) -> Result<String> {
        let path = Path::new("/sys/block")
            .join(device_path.trim_start_matches("/dev/"))
            .join("dev");
        let dev = fs::read_to_string(path)
            .map_err(|_| IOMaxError::InvalidDevice)?
            .trim()
            .to_string();
        if dev.is_empty() {
            Err(IOMaxError::InvalidDevice)
        } else {
            Ok(dev)
        }
    }

    // Validates the format of the device limit specification and transforms it into the required format.
    fn validate_and_transform(s: &str) -> Result<String> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(IOMaxError::InvalidFormat);
        }
        let major_minor = Self::device_path_to_major_minor(parts[0])?;
        parts[1]
            .parse::<u64>()
            .map_err(|_| IOMaxError::InvalidLimit)?;

        Ok(format!("{}:{}", major_minor, parts[1]))
    }

    pub fn device_read_bps(&self) -> Option<&String> {
        self.device_read_bps.as_ref()
    }

    pub fn device_write_bps(&self) -> Option<&String> {
        self.device_write_bps.as_ref()
    }

    pub fn device_read_iops(&self) -> Option<&String> {
        self.device_read_iops.as_ref()
    }

    pub fn device_write_iops(&self) -> Option<&String> {
        self.device_write_iops.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_input() {
        let result = IOMax::build(
            Some("/dev/sda:1000000".into()),
            Some("/dev/sdb:2000000".into()),
            Some("/dev/sda:1000".into()),
            Some("/dev/sdb:2000".into()),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn invalid_format_no_colon() {
        let result = IOMax::build(Some("/dev/sda1000000".into()), None, None, None);
        assert!(matches!(result, Err(IOMaxError::InvalidFormat)));
    }

    #[test]
    fn invalid_format_empty_value() {
        let result = IOMax::build(Some("/dev/sda:".into()), None, None, None);
        assert!(matches!(result, Err(IOMaxError::InvalidLimit)));
    }

    #[test]
    fn invalid_device() {
        let result = IOMax::build(Some("/dev/nonexistent:1000".into()), None, None, None);
        assert!(matches!(result, Err(IOMaxError::InvalidDevice)));
    }

    #[test]
    fn invalid_limit_non_numeric() {
        let result = IOMax::build(Some("/dev/sda:abc".into()), None, None, None);
        assert!(matches!(result, Err(IOMaxError::InvalidLimit)));
    }

    #[test]
    fn negative_limit() {
        let result = IOMax::build(Some("/dev/sda:-1000".into()), None, None, None);
        assert!(matches!(result, Err(IOMaxError::InvalidLimit)));
    }
}
