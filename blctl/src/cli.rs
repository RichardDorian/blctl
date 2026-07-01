use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "blctl", version, about = "Control Linux backlight devices")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
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
