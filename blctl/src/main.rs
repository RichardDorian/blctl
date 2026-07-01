use std::process::ExitCode;

use bllib::drivers::sysfs::SysfsDriver;
use bllib::{BacklightDriver, BacklightError};
use clap::{Parser, Subcommand};

/// Hardcoded for now -- device discovery/selection is future work.
const DEFAULT_DEVICE: &str = "intel_backlight";

#[derive(Parser)]
#[command(name = "blctl", version, about = "Control Linux backlight devices")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the maximum brightness value
    Max,
    /// Print the current brightness value
    Get,
    /// Set the brightness to VALUE
    Set {
        /// Brightness value, in the device's raw units
        value: u32,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let driver = match SysfsDriver::new(DEFAULT_DEVICE) {
        Ok(driver) => driver,
        Err(err) => return report_error(&err),
    };

    let result = match cli.command {
        Command::Max => driver.get_max_brightness().map(|v| println!("{v}")),
        Command::Get => driver.get_brightness().map(|v| println!("{v}")),
        Command::Set { value } => driver.set_brightness(value),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => report_error(&err),
    }
}

fn report_error(err: &BacklightError) -> ExitCode {
    eprintln!("blctl: {err}");
    ExitCode::FAILURE
}
