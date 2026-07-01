use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::CliError;

fn state_file_path() -> PathBuf {
    PathBuf::from("/tmp/blctl.toml")
}

fn read_state(path: &Path) -> Result<HashMap<String, u32>, CliError> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            return Ok(HashMap::new());
        }
        Err(source) => {
            return Err(CliError::StateRead {
                path: path.to_path_buf(),
                source,
            });
        }
    };

    toml::from_str(&contents).map_err(|source| CliError::StateParse {
        path: path.to_path_buf(),
        source,
    })
}

fn write_state(path: &Path, state: &HashMap<String, u32>) -> Result<(), CliError> {
    let contents =
        toml::to_string_pretty(state).map_err(|source| CliError::StateSerialize { source })?;

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .and_then(|mut file| file.write_all(contents.as_bytes()))
        .map_err(|source| CliError::StateWrite {
            path: path.to_path_buf(),
            source,
        })
}

fn save_at(path: &Path, device_id: &str, value: u32) -> Result<(), CliError> {
    let mut state = read_state(path)?;
    state.insert(device_id.to_string(), value);
    write_state(path, &state)
}

fn get_at(path: &Path, device_id: &str) -> Result<Option<u32>, CliError> {
    Ok(read_state(path)?.get(device_id).copied())
}

fn remove_at(path: &Path, device_id: &str) -> Result<(), CliError> {
    let mut state = read_state(path)?;
    if state.remove(device_id).is_some() {
        write_state(path, &state)?;
    }
    Ok(())
}

/// Saves `value` as the brightness to restore later for `device_id`.
pub fn save(device_id: &str, value: u32) -> Result<(), CliError> {
    save_at(&state_file_path(), device_id, value)
}

/// Returns the saved brightness for `device_id`, if any.
pub fn get(device_id: &str) -> Result<Option<u32>, CliError> {
    get_at(&state_file_path(), device_id)
}

/// Removes the saved brightness entry for `device_id`, if any.
pub fn remove(device_id: &str) -> Result<(), CliError> {
    remove_at(&state_file_path(), device_id)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;

    /// A path under the system temp dir unique to this test invocation, so
    /// parallel tests don't clobber each other's state.
    fn scratch_path() -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("blctl-test-{}-{n}.toml", std::process::id()))
    }

    #[test]
    fn get_returns_none_when_file_is_missing() {
        let path = scratch_path();
        assert_eq!(get_at(&path, "acpi:intel_backlight").unwrap(), None);
    }

    #[test]
    fn save_then_get_round_trips() {
        let path = scratch_path();
        save_at(&path, "acpi:intel_backlight", 42).unwrap();
        assert_eq!(get_at(&path, "acpi:intel_backlight").unwrap(), Some(42));
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn save_does_not_disturb_other_devices() {
        let path = scratch_path();
        save_at(&path, "acpi:intel_backlight", 42).unwrap();
        save_at(&path, "acpi:other", 7).unwrap();
        assert_eq!(get_at(&path, "acpi:intel_backlight").unwrap(), Some(42));
        assert_eq!(get_at(&path, "acpi:other").unwrap(), Some(7));
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn save_overwrites_previous_value_for_same_device() {
        let path = scratch_path();
        save_at(&path, "acpi:intel_backlight", 42).unwrap();
        save_at(&path, "acpi:intel_backlight", 99).unwrap();
        assert_eq!(get_at(&path, "acpi:intel_backlight").unwrap(), Some(99));
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn remove_deletes_the_entry() {
        let path = scratch_path();
        save_at(&path, "acpi:intel_backlight", 42).unwrap();
        remove_at(&path, "acpi:intel_backlight").unwrap();
        assert_eq!(get_at(&path, "acpi:intel_backlight").unwrap(), None);
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn remove_is_a_no_op_when_file_is_missing() {
        let path = scratch_path();
        remove_at(&path, "acpi:intel_backlight").unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn remove_is_a_no_op_when_device_is_not_saved() {
        let path = scratch_path();
        save_at(&path, "acpi:other", 7).unwrap();
        remove_at(&path, "acpi:intel_backlight").unwrap();
        assert_eq!(get_at(&path, "acpi:other").unwrap(), Some(7));
        fs::remove_file(&path).unwrap();
    }
}
