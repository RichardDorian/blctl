use crate::driver::{BacklightDriver, BacklightError};

/// Discovers backlight devices controllable by a particular driver backend.
///
/// Each driver backend (e.g. sysfs) has its own scanner implementation,
/// returning one driver instance per device it finds.
pub trait DeviceScanner {
    /// The concrete driver type this scanner discovers devices for.
    type Driver: BacklightDriver;

    /// Discover all devices currently available for this driver backend.
    fn scan(&self) -> Result<Vec<Self::Driver>, BacklightError>;
}
