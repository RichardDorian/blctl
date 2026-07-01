use bllib::{BacklightDriver, BacklightError};

pub fn run(driver: &impl BacklightDriver) -> Result<(), BacklightError> {
    driver.get_brightness().map(|v| println!("{v}"))
}
