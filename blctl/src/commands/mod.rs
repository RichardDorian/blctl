pub mod completions;
mod get;
pub mod list;
mod max;
mod set;

use bllib::{BacklightDriver, BacklightError};

use crate::cli::Command;

pub fn run(command: Command, driver: &dyn BacklightDriver) -> Result<(), BacklightError> {
    match command {
        Command::Max { .. } => max::run(driver),
        Command::Get { .. } => get::run(driver),
        Command::Set { value, .. } => set::run(driver, value),
        // Handled in main() before a single device is selected, these
        // commands don't operate on one backlight device.
        Command::Completions { .. } | Command::List => unreachable!(),
    }
}
