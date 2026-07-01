use bllib::{BacklightDriver, BacklightError};

pub fn run(devices: &[Box<dyn BacklightDriver>]) -> Result<(), BacklightError> {
    for (i, device) in devices.iter().enumerate() {
        if i > 0 {
            println!();
        }
        let brightness = device.get_brightness()?;
        let max_brightness = device.get_max_brightness()?;
        println!("{} ({})", device.name(), device.driver_name());
        println!("  brightness: {brightness} / {max_brightness}");
    }
    Ok(())
}
