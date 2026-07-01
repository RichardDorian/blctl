use bllib::transitions::Exponential;
use bllib::{BacklightDriver, TransitionConfig, transition_brightness};

use crate::error::CliError;
use crate::utils::{device_id, state_store};

pub fn run(driver: &dyn BacklightDriver) -> Result<(), CliError> {
    let id = device_id(driver);

    let Some(value) = state_store::get(&id)? else {
        return Err(CliError::NoSavedBrightness { device: id });
    };

    transition_brightness(
        driver,
        value,
        &Exponential::default(),
        &TransitionConfig::default(),
    )?;

    state_store::remove(&id)
}
