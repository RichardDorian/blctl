use crate::transition::Transition;

/// A transition that follows an exponential curve, controlled by `rate`.
///
/// Positive rates accelerate towards the target (slow start, fast finish);
/// negative rates decelerate (fast start, slow finish). A rate of `0.0`
/// behaves like a linear transition.
#[derive(Debug, Clone, Copy)]
pub struct Exponential {
    rate: f64,
}

impl Exponential {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl Default for Exponential {
    /// A moderate acceleration curve (slow start, fast finish).
    fn default() -> Self {
        Self::new(5.0)
    }
}

impl Transition for Exponential {
    fn ease(&self, t: f64) -> f64 {
        if self.rate.abs() < f64::EPSILON {
            return t;
        }
        (self.rate * t).exp_m1() / self.rate.exp_m1()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_match_linear_regardless_of_rate() {
        for rate in [-8.0, -1.0, 1.0, 8.0] {
            let exp = Exponential::new(rate);
            assert!((exp.ease(0.0) - 0.0).abs() < 1e-9, "rate {rate}");
            assert!((exp.ease(1.0) - 1.0).abs() < 1e-9, "rate {rate}");
        }
    }

    #[test]
    fn zero_rate_is_linear() {
        let exp = Exponential::new(0.0);
        assert_eq!(exp.ease(0.25), 0.25);
        assert_eq!(exp.ease(0.75), 0.75);
    }

    #[test]
    fn positive_rate_accelerates_towards_the_end() {
        let exp = Exponential::default();
        assert!(exp.ease(0.5) < 0.5, "midpoint should lag behind linear");
    }

    #[test]
    fn negative_rate_decelerates_towards_the_end() {
        let exp = Exponential::new(-5.0);
        assert!(exp.ease(0.5) > 0.5, "midpoint should lead linear");
    }
}
