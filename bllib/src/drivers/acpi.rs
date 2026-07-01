use std::path::Path;
use std::{fs, io, path::PathBuf};

use crate::discovery::DeviceScanner;
use crate::driver::{BacklightDriver, BacklightError};

const ACPI_BACKLIGHT_ROOT: &str = "/sys/class/backlight";

/// Backlight driver that talks to a device under
/// `/sys/class/backlight/<device>`.
#[derive(Debug)]
pub struct AcpiDriver {
    device: String,
    path: PathBuf,
}

impl AcpiDriver {
    /// Resolve `device_name` to `/sys/class/backlight/<device_name>` and
    /// verify it exists.
    pub fn new(device_name: &str) -> Result<Self, BacklightError> {
        let path = PathBuf::from(ACPI_BACKLIGHT_ROOT).join(device_name);
        if !path.is_dir() {
            return Err(BacklightError::NotFound {
                device: device_name.to_string(),
                path,
            });
        }
        Ok(Self {
            device: device_name.to_string(),
            path,
        })
    }

    fn read_u32(&self, filename: &str) -> Result<u32, BacklightError> {
        let file_path = self.path.join(filename);
        let contents = fs::read_to_string(&file_path).map_err(|source| BacklightError::Io {
            device: self.device.clone(),
            source,
        })?;
        contents.trim().parse::<u32>().map_err(|parse_err| {
            let source = io::Error::new(io::ErrorKind::InvalidData, parse_err);
            BacklightError::Io {
                device: self.device.clone(),
                source,
            }
        })
    }
}

impl BacklightDriver for AcpiDriver {
    fn name(&self) -> &str {
        &self.device
    }

    fn driver_name(&self) -> &'static str {
        "ACPI"
    }

    fn get_max_brightness(&self) -> Result<u32, BacklightError> {
        self.read_u32("max_brightness")
    }

    fn get_brightness(&self) -> Result<u32, BacklightError> {
        // Read `brightness` (the requested/applied value), not
        // `actual_brightness` (hardware-reported, may lag during fades).
        // This keeps get/set symmetric: get() right after set(v) returns v.
        self.read_u32("brightness")
    }

    fn set_brightness(&self, value: u32) -> Result<(), BacklightError> {
        let max = self.get_max_brightness()?;
        if value > max {
            return Err(BacklightError::InvalidValue {
                device: self.device.clone(),
                value,
                max,
            });
        }

        let file_path = self.path.join("brightness");
        fs::write(&file_path, value.to_string()).map_err(|source| {
            if source.kind() == io::ErrorKind::PermissionDenied {
                BacklightError::PermissionDenied {
                    device: self.device.clone(),
                    source,
                }
            } else {
                BacklightError::Io {
                    device: self.device.clone(),
                    source,
                }
            }
        })
    }
}

/// Discovers ACPI backlight devices under `/sys/class/backlight`.
#[derive(Debug, Default, Clone, Copy)]
pub struct AcpiScanner;

impl AcpiScanner {
    pub fn new() -> Self {
        Self
    }
}

impl DeviceScanner for AcpiScanner {
    type Driver = AcpiDriver;

    fn scan(&self) -> Result<Vec<AcpiDriver>, BacklightError> {
        let root = Path::new(ACPI_BACKLIGHT_ROOT);
        if !root.is_dir() {
            // No backlight class on this system - not an error, just no devices.
            return Ok(Vec::new());
        }

        let to_io_err = |source: io::Error| BacklightError::Io {
            device: ACPI_BACKLIGHT_ROOT.to_string(),
            source,
        };

        fs::read_dir(root)
            .map_err(to_io_err)?
            .map(|entry| {
                let entry = entry.map_err(to_io_err)?;
                AcpiDriver::new(&entry.file_name().to_string_lossy())
            })
            .collect()
    }
}
