pub mod completions;
mod get;
pub mod list;
mod max;
mod restore;
mod set;

use bllib::BacklightDriver;

use crate::cli::Command;
use crate::error::CliError;

pub fn run(command: Command, driver: &dyn BacklightDriver) -> Result<(), CliError> {
    match command {
        Command::Max { .. } => max::run(driver).map_err(CliError::from),
        Command::Get { .. } => get::run(driver).map_err(CliError::from),
        Command::Set {
            value,
            immediate,
            save,
            ..
        } => set::run(driver, value, immediate, save),
        Command::Restore { .. } => restore::run(driver),
        // Handled in main() before a single device is selected, these
        // commands don't operate on one backlight device.
        Command::Completions { .. } | Command::List => unreachable!(),
    }
}
