mod cli;
mod commands;

use std::process::ExitCode;

use bllib::drivers::sysfs::SysfsScanner;
use bllib::{BacklightError, DeviceScanner};
use clap::Parser;
use cli::Cli;

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

    match commands::run(cli.command, &driver) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => report_error(&err),
    }
}

fn report_error(err: &BacklightError) -> ExitCode {
    eprintln!("blctl: {err}");
    ExitCode::FAILURE
}
