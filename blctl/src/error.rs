use std::io;
use std::path::PathBuf;

use bllib::BacklightError;

/// Errors that can occur while running a `blctl` command, including
/// failures accessing the saved-brightness state file used by
/// `set --save` and `restore`.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(transparent)]
    Backlight(#[from] BacklightError),

    #[error("no saved brightness found for device '{device}' (use 'set --save' first)")]
    NoSavedBrightness { device: String },

    #[error("failed to read saved brightness state at {path}: {source}", path = path.display())]
    StateRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to write saved brightness state at {path}: {source}", path = path.display())]
    StateWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse saved brightness state at {path}: {source}", path = path.display())]
    StateParse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("failed to serialize saved brightness state: {source}")]
    StateSerialize {
        #[source]
        source: toml::ser::Error,
    },
}
