use std::thread;
use std::time::Duration;

use crate::driver::{BacklightDriver, BacklightError};

/// A mathematical curve describing how brightness moves from a starting
/// value to a target value over time.
///
/// Implementations map a linear progress value `t` in `[0.0, 1.0]` (elapsed
/// fraction of the transition's duration) to an eased progress value, also
/// expected to be in `[0.0, 1.0]`, with `ease(0.0) == 0.0` and
/// `ease(1.0) == 1.0`.
pub trait Transition {
    fn ease(&self, t: f64) -> f64;
}

/// Timing parameters for a brightness transition.
#[derive(Debug, Clone, Copy)]
pub struct TransitionConfig {
    /// Total time the transition should take.
    pub duration: Duration,
    /// Time to wait between successive brightness updates.
    pub step_interval: Duration,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(250),
            step_interval: Duration::from_millis(16), // 1/60 fps = 0.0166.. seconmds ≈ 16ms
        }
    }
}

/// Move `driver`'s brightness from its current value to `target`, following
/// `transition`'s curve over `config.duration`.
///
/// Blocks the calling thread for the duration of the transition, sleeping
/// `config.step_interval` between brightness updates.
pub fn transition_brightness(
    driver: &dyn BacklightDriver,
    target: u32,
    transition: &impl Transition,
    config: &TransitionConfig,
) -> Result<(), BacklightError> {
    let max = driver.get_max_brightness()?;
    if target > max {
        return Err(BacklightError::InvalidValue {
            device: driver.name().to_string(),
            value: target,
            max,
        });
    }

    let start = driver.get_brightness()?;
    if start == target {
        return Ok(());
    }

    let steps = (config.duration.as_secs_f64() / config.step_interval.as_secs_f64())
        .round()
        .max(1.0) as u64;

    for step in 1..=steps {
        let t = step as f64 / steps as f64;
        let eased = transition.ease(t).clamp(0.0, 1.0);
        let value = start as f64 + (target as f64 - start as f64) * eased;
        driver.set_brightness(value.round() as u32)?;

        if step < steps {
            thread::sleep(config.step_interval);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};

    use super::*;
    use crate::transitions::Linear;

    struct MockDriver {
        max: u32,
        brightness: Cell<u32>,
        history: RefCell<Vec<u32>>,
    }

    impl MockDriver {
        fn new(brightness: u32, max: u32) -> Self {
            Self {
                max,
                brightness: Cell::new(brightness),
                history: RefCell::new(Vec::new()),
            }
        }
    }

    impl BacklightDriver for MockDriver {
        fn name(&self) -> &str {
            "mock"
        }

        fn driver_name(&self) -> &'static str {
            "mock"
        }

        fn get_brightness(&self) -> Result<u32, BacklightError> {
            Ok(self.brightness.get())
        }

        fn get_max_brightness(&self) -> Result<u32, BacklightError> {
            Ok(self.max)
        }

        fn set_brightness(&self, value: u32) -> Result<(), BacklightError> {
            self.brightness.set(value);
            self.history.borrow_mut().push(value);
            Ok(())
        }
    }

    fn fast_config() -> TransitionConfig {
        TransitionConfig {
            duration: Duration::from_millis(20),
            step_interval: Duration::from_millis(5),
        }
    }

    #[test]
    fn transitions_from_start_to_target() {
        let driver = MockDriver::new(0, 100);
        transition_brightness(&driver, 100, &Linear, &fast_config()).unwrap();

        assert_eq!(driver.get_brightness().unwrap(), 100);
        assert_eq!(driver.history.borrow().last(), Some(&100));
    }

    #[test]
    fn is_a_no_op_when_already_at_target() {
        let driver = MockDriver::new(50, 100);
        transition_brightness(&driver, 50, &Linear, &fast_config()).unwrap();

        assert!(driver.history.borrow().is_empty());
    }

    #[test]
    fn rejects_target_above_max() {
        let driver = MockDriver::new(0, 100);
        let err = transition_brightness(&driver, 200, &Linear, &fast_config()).unwrap_err();

        assert!(matches!(
            err,
            BacklightError::InvalidValue {
                value: 200,
                max: 100,
                ..
            }
        ));
        assert!(driver.history.borrow().is_empty());
    }
}
