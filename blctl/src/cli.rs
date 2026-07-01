use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::utils::{BrightnessValue, parse_brightness_value};

#[derive(Parser)]
#[command(name = "blctl", version, about = "Control Linux backlight devices")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// List all detected backlight devices
    List,
    /// Print the maximum brightness value
    Max {
        /// Name of the backlight device to operate on
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Print the current brightness value
    Get {
        /// Name of the backlight device to operate on
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Set the brightness to VALUE
    Set {
        /// Brightness value, either in the device's raw units or, if
        /// suffixed with '%', as a percentage of the device's maximum
        /// brightness (e.g. '50%'). Prefix with '+' or '-' to adjust the
        /// current brightness by that amount instead of setting it
        /// outright (e.g. '+10%', '-5')
        #[arg(value_parser = parse_brightness_value, allow_hyphen_values = true)]
        value: BrightnessValue,
        /// Name of the backlight device to operate on
        #[arg(short, long)]
        device: Option<String>,
        /// Set the brightness immediately, without a smooth transition
        #[arg(short, long)]
        immediate: bool,
    },
    /// Generate a shell completion script, printed to stdout
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    },
}

impl Command {
    /// The `--device` argument given to this command, if it accepts one.
    pub fn device(&self) -> Option<&str> {
        match self {
            Command::Max { device } | Command::Get { device } | Command::Set { device, .. } => {
                device.as_deref()
            }
            Command::List | Command::Completions { .. } => None,
        }
    }
}
