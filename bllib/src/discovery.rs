use crate::driver::{BacklightDriver, BacklightError};

/// Discovers backlight devices controllable by a particular driver backend.
///
/// Each driver backend (e.g. ACPI) has its own scanner implementation,
/// returning one driver instance per device it finds.
pub trait DeviceScanner {
    /// The concrete driver type this scanner discovers devices for.
    type Driver: BacklightDriver + 'static;

    /// Discover all devices currently available for this driver backend.
    fn scan(&self) -> Result<Vec<Self::Driver>, BacklightError>;

    /// Like [`scan`](Self::scan), but with each device boxed as a
    /// `dyn BacklightDriver` so it can be mixed with devices from other
    /// driver backends.
    fn scan_boxed(&self) -> Result<Vec<Box<dyn BacklightDriver>>, BacklightError> {
        Ok(self
            .scan()?
            .into_iter()
            .map(|driver| Box::new(driver) as Box<dyn BacklightDriver>)
            .collect())
    }
}

/// Discover backlight devices across every supported driver backend.
///
/// New backends (e.g. DDC) are added here alongside the existing ones.
pub fn scan_all_devices() -> Result<Vec<Box<dyn BacklightDriver>>, BacklightError> {
    let mut devices = Vec::new();
    devices.extend(crate::drivers::acpi::AcpiScanner::new().scan_boxed()?);
    Ok(devices)
}
