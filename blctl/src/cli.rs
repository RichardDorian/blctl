use clap::{Parser, Subcommand};
use clap_complete::Shell;

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
        /// brightness (e.g. '50%')
        #[arg(value_parser = parse_brightness_value)]
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

/// A brightness value given on the command line, either as raw device
/// units or as a percentage of the device's maximum brightness.
#[derive(Debug, Clone, Copy)]
pub enum BrightnessValue {
    Raw(u32),
    Percent(f64),
}

impl BrightnessValue {
    /// Resolve this value to raw device units, given the device's maximum
    /// brightness.
    pub fn resolve(self, max: u32) -> u32 {
        match self {
            BrightnessValue::Raw(value) => value,
            BrightnessValue::Percent(percent) => (max as f64 * percent / 100.0).round() as u32,
        }
    }
}

fn parse_brightness_value(s: &str) -> Result<BrightnessValue, String> {
    match s.strip_suffix('%') {
        Some(percent) => {
            let percent: f64 = percent
                .parse()
                .map_err(|_| format!("invalid brightness percentage '{s}'"))?;
            if !percent.is_finite() || percent < 0.0 {
                return Err(format!("invalid brightness percentage '{s}'"));
            }
            Ok(BrightnessValue::Percent(percent))
        }
        None => s
            .parse::<u32>()
            .map(BrightnessValue::Raw)
            .map_err(|_| format!("invalid brightness value '{s}'")),
    }
}
