mod cli;
mod commands;
mod error;
mod utils;

use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Command::Completions { shell } = cli.command {
        commands::completions::run(shell);
        return ExitCode::SUCCESS;
    }

    let devices = match bllib::scan_all_devices() {
        Ok(devices) => devices,
        Err(err) => return report_error(&err),
    };

    if let Command::List = cli.command {
        return match commands::list::run(&devices) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => report_error(&err),
        };
    }

    let driver = match cli.command.device() {
        Some(name) => match devices.into_iter().find(|d| d.name() == name) {
            Some(driver) => driver,
            None => {
                eprintln!("blctl: no backlight device named '{name}' found");
                return ExitCode::FAILURE;
            }
        },
        None => {
            let Some(driver) = devices.into_iter().next() else {
                eprintln!("blctl: no backlight devices found");
                return ExitCode::FAILURE;
            };
            driver
        }
    };

    match commands::run(cli.command, driver.as_ref()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => report_error(&err),
    }
}

fn report_error(err: &impl std::fmt::Display) -> ExitCode {
    eprintln!("blctl: {err}");
    ExitCode::FAILURE
}
