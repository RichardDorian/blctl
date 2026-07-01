use std::io;
use std::path::PathBuf;

/// A pluggable way of sending backlight commands to a device.
///
/// All brightness values are raw device units as reported by the driver's
/// underlying interface (for the sysfs driver, this is whatever scale the
/// kernel exposes via `max_brightness` -- it is NOT normalized to 0-100).
pub trait BacklightDriver {
    /// Identifier of the device this driver instance targets.
    fn name(&self) -> &str;

    /// Identifier of the driver backend used to control this device (e.g.
    /// `"sysfs"`).
    fn driver_name(&self) -> &'static str;

    /// Current brightness, in raw device units.
    fn get_brightness(&self) -> Result<u32, BacklightError>;

    /// Maximum brightness accepted by this device, in raw device units.
    fn get_max_brightness(&self) -> Result<u32, BacklightError>;

    /// Set brightness to `value` (raw device units).
    fn set_brightness(&self, value: u32) -> Result<(), BacklightError>;
}

/// Errors produced by a [`BacklightDriver`].
#[derive(Debug, thiserror::Error)]
pub enum BacklightError {
    /// The targeted device does not exist.
    #[error("backlight device '{device}' not found (expected at {path})", path = path.display())]
    NotFound { device: String, path: PathBuf },

    /// The underlying interface refused the write due to permissions.
    #[error(
        "permission denied setting brightness on '{device}': {source} \
         (writing brightness usually requires root or membership in the \
         udev-granted group, e.g. 'video')"
    )]
    PermissionDenied {
        device: String,
        #[source]
        source: io::Error,
    },

    /// The requested value is not valid for this device (e.g. out of range).
    #[error("invalid brightness value {value} for '{device}' (must be 0..={max})")]
    InvalidValue {
        device: String,
        value: u32,
        max: u32,
    },

    /// Any other I/O failure talking to the device.
    #[error("I/O error accessing backlight device '{device}': {source}")]
    Io {
        device: String,
        #[source]
        source: io::Error,
    },
}
