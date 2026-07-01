mod discovery;
mod driver;
pub mod drivers;
mod transition;
pub mod transitions;

pub use discovery::DeviceScanner;
pub use driver::{BacklightDriver, BacklightError};
pub use transition::{Transition, TransitionConfig, transition_brightness};
