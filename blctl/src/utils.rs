mod brightness_value;
pub mod state_store;

use bllib::BacklightDriver;

pub use brightness_value::{BrightnessValue, parse_brightness_value};

/// Identifier used to key saved-brightness state, of the form
/// `<driver>:<device>` (e.g. `acpi:intel_backlight`).
pub fn device_id(driver: &dyn BacklightDriver) -> String {
    format!("{}:{}", driver.driver_name().to_lowercase(), driver.name())
}
