use crate::transition::Transition;

/// A transition that moves at a constant rate from start to target.
#[derive(Debug, Default, Clone, Copy)]
pub struct Linear;

impl Transition for Linear {
    fn ease(&self, t: f64) -> f64 {
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_the_identity_function() {
        assert_eq!(Linear.ease(0.0), 0.0);
        assert_eq!(Linear.ease(0.5), 0.5);
        assert_eq!(Linear.ease(1.0), 1.0);
    }
}
