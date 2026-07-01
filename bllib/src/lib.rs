mod driver;
pub mod drivers;
mod transition;
pub mod transitions;

pub use driver::{BacklightDriver, BacklightError};
pub use transition::{transition_brightness, Transition, TransitionConfig};
