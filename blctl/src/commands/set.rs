use bllib::transitions::Exponential;
use bllib::{BacklightDriver, BacklightError, TransitionConfig, transition_brightness};

pub fn run(
    driver: &dyn BacklightDriver,
    value: u32,
    immediate: bool,
) -> Result<(), BacklightError> {
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
