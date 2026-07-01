use std::process::ExitCode;

use bllib::drivers::sysfs::SysfsScanner;
use bllib::transitions::Exponential;
use bllib::{
    BacklightDriver, BacklightError, DeviceScanner, TransitionConfig, transition_brightness,
};
use clap::{Parser, Subcommand};

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

    let devices = match SysfsScanner::new().scan() {
        Ok(devices) => devices,
        Err(err) => return report_error(&err),
    };

    let Some(driver) = devices.into_iter().next() else {
        eprintln!("blctl: no backlight devices found");
        return ExitCode::FAILURE;
    };

    let result = match cli.command {
        Command::Max => driver.get_max_brightness().map(|v| println!("{v}")),
        Command::Get => driver.get_brightness().map(|v| println!("{v}")),
        Command::Set { value } => transition_brightness(
            &driver,
            value,
            &Exponential::default(),
            &TransitionConfig::default(),
        ),
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
