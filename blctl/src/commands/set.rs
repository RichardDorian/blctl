use bllib::transitions::Exponential;
use bllib::{BacklightDriver, BacklightError, TransitionConfig, transition_brightness};

pub fn run(driver: &impl BacklightDriver, value: u32) -> Result<(), BacklightError> {
    transition_brightness(
        driver,
        value,
        &Exponential::default(),
        &TransitionConfig::default(),
    )
}
