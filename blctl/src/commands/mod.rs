mod get;
mod max;
mod set;

use bllib::{BacklightDriver, BacklightError};

use crate::cli::Command;

pub fn run(command: Command, driver: &impl BacklightDriver) -> Result<(), BacklightError> {
    match command {
        Command::Max => max::run(driver),
        Command::Get => get::run(driver),
        Command::Set { value } => set::run(driver, value),
    }
}
