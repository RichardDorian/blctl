use bllib::transitions::Exponential;
use bllib::{BacklightDriver, BacklightError, TransitionConfig, transition_brightness};

use crate::cli::BrightnessValue;

pub fn run(
    driver: &dyn BacklightDriver,
    value: BrightnessValue,
    immediate: bool,
) -> Result<(), BacklightError> {
    let value = value.resolve(driver.get_max_brightness()?);

    if immediate {
        return driver.set_brightness(value);
    }

    transition_brightness(
        driver,
        value,
        &Exponential::default(),
        &TransitionConfig::default(),
    )
}
