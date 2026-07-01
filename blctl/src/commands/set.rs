use bllib::transitions::Exponential;
use bllib::{BacklightDriver, TransitionConfig, transition_brightness};

use crate::error::CliError;
use crate::utils::{BrightnessValue, device_id, state_store};

pub fn run(
    driver: &dyn BacklightDriver,
    value: BrightnessValue,
    immediate: bool,
    save: bool,
) -> Result<(), CliError> {
    let current = driver.get_brightness()?;
    let value = value.resolve(current, driver.get_max_brightness()?);

    if save {
        state_store::save(&device_id(driver), current)?;
    }

    if immediate {
        return driver.set_brightness(value).map_err(CliError::from);
    }

    transition_brightness(
        driver,
        value,
        &Exponential::default(),
        &TransitionConfig::default(),
    )?;
    Ok(())
}
