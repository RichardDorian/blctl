/// A brightness value given on the command line, either as raw device
/// units or as a percentage of the device's maximum brightness, and either
/// an absolute value or a delta applied to the current brightness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrightnessValue {
    Raw(u32),
    Percent(f64),
    DeltaRaw(i64),
    DeltaPercent(f64),
}

impl BrightnessValue {
    /// Resolve this value to raw device units, given the device's current
    /// and maximum brightness. Delta values are clamped to `0..=max`
    /// rather than over/underflowing or erroring.
    ///
    /// Percentage-derived values never resolve to 0 (unless `max` itself
    /// is 0) - a percentage is meant to dim the backlight, not turn it
    /// off, so anything that rounds down to 0 is floored to 1 instead.
    pub fn resolve(self, current: u32, max: u32) -> u32 {
        match self {
            BrightnessValue::Raw(value) => value,
            BrightnessValue::Percent(percent) => {
                floor_percent_result((max as f64 * percent / 100.0).round(), max)
            }
            BrightnessValue::DeltaRaw(delta) => {
                (current as i64).saturating_add(delta).clamp(0, max as i64) as u32
            }
            BrightnessValue::DeltaPercent(percent) => {
                let delta = max as f64 * percent / 100.0;
                let value = (current as f64 + delta).round().clamp(0.0, max as f64);
                floor_percent_result(value, max)
            }
        }
    }
}

/// Floors a percentage-derived brightness value at 1 instead of 0, unless
/// the device's maximum brightness is itself 0 (in which case 0 is the
/// only value that isn't out of range).
fn floor_percent_result(value: f64, max: u32) -> u32 {
    if value < 1.0 && max >= 1 {
        1
    } else {
        value as u32
    }
}

/// Parses a [`BrightnessValue`] from a command-line argument, for use as a
/// clap `value_parser`.
pub fn parse_brightness_value(s: &str) -> Result<BrightnessValue, String> {
    if s.eq_ignore_ascii_case("max") {
        return Ok(BrightnessValue::Percent(100.0));
    }
    if s.eq_ignore_ascii_case("min") {
        return Ok(BrightnessValue::Percent(0.0));
    }

    let (sign, rest) = match s.strip_prefix('+') {
        Some(rest) => (Some(1.0), rest),
        None => match s.strip_prefix('-') {
            Some(rest) => (Some(-1.0), rest),
            None => (None, s),
        },
    };

    if rest.starts_with('+') || rest.starts_with('-') {
        return Err(format!("invalid brightness value '{s}'"));
    }

    match rest.strip_suffix('%') {
        Some(percent) => {
            let percent: f64 = percent
                .parse()
                .map_err(|_| format!("invalid brightness percentage '{s}'"))?;
            if !percent.is_finite() || percent < 0.0 {
                return Err(format!("invalid brightness percentage '{s}'"));
            }
            Ok(match sign {
                Some(sign) => BrightnessValue::DeltaPercent(sign * percent),
                None => BrightnessValue::Percent(percent),
            })
        }
        None => {
            let value: u32 = rest
                .parse()
                .map_err(|_| format!("invalid brightness value '{s}'"))?;
            Ok(match sign {
                Some(sign) => BrightnessValue::DeltaRaw(sign as i64 * value as i64),
                None => BrightnessValue::Raw(value),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_brightness_value ---

    #[test]
    fn parses_raw_value() {
        assert_eq!(parse_brightness_value("50"), Ok(BrightnessValue::Raw(50)));
    }

    #[test]
    fn parses_zero() {
        assert_eq!(parse_brightness_value("0"), Ok(BrightnessValue::Raw(0)));
    }

    #[test]
    fn parses_percent_value() {
        assert_eq!(
            parse_brightness_value("50%"),
            Ok(BrightnessValue::Percent(50.0))
        );
    }

    #[test]
    fn parses_fractional_percent_value() {
        assert_eq!(
            parse_brightness_value("12.5%"),
            Ok(BrightnessValue::Percent(12.5))
        );
    }

    #[test]
    fn parses_max_alias_as_100_percent() {
        assert_eq!(
            parse_brightness_value("max"),
            Ok(BrightnessValue::Percent(100.0))
        );
    }

    #[test]
    fn parses_max_alias_case_insensitively() {
        assert_eq!(
            parse_brightness_value("MAX"),
            Ok(BrightnessValue::Percent(100.0))
        );
        assert_eq!(
            parse_brightness_value("Max"),
            Ok(BrightnessValue::Percent(100.0))
        );
    }

    #[test]
    fn parses_min_alias_as_0_percent() {
        assert_eq!(
            parse_brightness_value("min"),
            Ok(BrightnessValue::Percent(0.0))
        );
    }

    #[test]
    fn parses_min_alias_case_insensitively() {
        assert_eq!(
            parse_brightness_value("MIN"),
            Ok(BrightnessValue::Percent(0.0))
        );
        assert_eq!(
            parse_brightness_value("Min"),
            Ok(BrightnessValue::Percent(0.0))
        );
    }

    #[test]
    fn parses_percent_value_above_100() {
        assert_eq!(
            parse_brightness_value("150%"),
            Ok(BrightnessValue::Percent(150.0))
        );
    }

    #[test]
    fn parses_positive_raw_delta() {
        assert_eq!(
            parse_brightness_value("+10"),
            Ok(BrightnessValue::DeltaRaw(10))
        );
    }

    #[test]
    fn parses_negative_raw_delta() {
        assert_eq!(
            parse_brightness_value("-10"),
            Ok(BrightnessValue::DeltaRaw(-10))
        );
    }

    #[test]
    fn parses_positive_percent_delta() {
        assert_eq!(
            parse_brightness_value("+10%"),
            Ok(BrightnessValue::DeltaPercent(10.0))
        );
    }

    #[test]
    fn parses_negative_percent_delta() {
        assert_eq!(
            parse_brightness_value("-10%"),
            Ok(BrightnessValue::DeltaPercent(-10.0))
        );
    }

    #[test]
    fn rejects_empty_string() {
        assert!(parse_brightness_value("").is_err());
    }

    #[test]
    fn rejects_sign_with_no_digits() {
        assert!(parse_brightness_value("+").is_err());
        assert!(parse_brightness_value("-").is_err());
    }

    #[test]
    fn rejects_percent_sign_with_no_digits() {
        assert!(parse_brightness_value("%").is_err());
        assert!(parse_brightness_value("+%").is_err());
    }

    #[test]
    fn rejects_non_numeric_value() {
        assert!(parse_brightness_value("abc").is_err());
    }

    #[test]
    fn rejects_non_numeric_percent() {
        assert!(parse_brightness_value("abc%").is_err());
    }

    #[test]
    fn rejects_negative_absolute_percent() {
        // A bare '-' is consumed as the delta sign, so this parses as a
        // percent delta rather than a negative absolute percentage - it
        // must never be rejected for being "negative".
        assert_eq!(
            parse_brightness_value("-5%"),
            Ok(BrightnessValue::DeltaPercent(-5.0))
        );
    }

    #[test]
    fn rejects_malformed_numeric_part() {
        assert!(parse_brightness_value("-1abc").is_err());
    }

    #[test]
    fn rejects_raw_value_overflowing_u32() {
        assert!(parse_brightness_value("99999999999999999999").is_err());
    }

    #[test]
    fn rejects_non_finite_percent() {
        assert!(parse_brightness_value("inf%").is_err());
        assert!(parse_brightness_value("nan%").is_err());
    }

    #[test]
    fn rejects_whitespace_only() {
        assert!(parse_brightness_value("   ").is_err());
    }

    #[test]
    fn rejects_double_sign() {
        assert!(parse_brightness_value("++10").is_err());
        assert!(parse_brightness_value("--10").is_err());
        assert!(parse_brightness_value("+-10").is_err());
        assert!(parse_brightness_value("-+10").is_err());
        assert!(parse_brightness_value("++10%").is_err());
    }

    // --- BrightnessValue::resolve ---

    #[test]
    fn resolves_raw_value_regardless_of_current() {
        assert_eq!(BrightnessValue::Raw(30).resolve(10, 100), 30);
    }

    #[test]
    fn resolves_percent_of_max() {
        assert_eq!(BrightnessValue::Percent(50.0).resolve(0, 200), 100);
    }

    #[test]
    fn resolves_percent_rounds_to_nearest() {
        assert_eq!(BrightnessValue::Percent(33.0).resolve(0, 100), 33);
        assert_eq!(BrightnessValue::Percent(33.5).resolve(0, 100), 34);
    }

    #[test]
    fn resolves_positive_raw_delta() {
        assert_eq!(BrightnessValue::DeltaRaw(10).resolve(50, 100), 60);
    }

    #[test]
    fn resolves_negative_raw_delta() {
        assert_eq!(BrightnessValue::DeltaRaw(-10).resolve(50, 100), 40);
    }

    #[test]
    fn clamps_positive_raw_delta_to_max() {
        assert_eq!(BrightnessValue::DeltaRaw(1000).resolve(50, 100), 100);
    }

    #[test]
    fn clamps_negative_raw_delta_to_zero() {
        assert_eq!(BrightnessValue::DeltaRaw(-1000).resolve(50, 100), 0);
    }

    #[test]
    fn clamps_raw_delta_without_overflowing_on_i64_min() {
        assert_eq!(BrightnessValue::DeltaRaw(i64::MIN).resolve(50, 100), 0);
    }

    #[test]
    fn clamps_raw_delta_without_overflowing_on_i64_max() {
        assert_eq!(BrightnessValue::DeltaRaw(i64::MAX).resolve(50, 100), 100);
    }

    #[test]
    fn clamps_raw_delta_at_max_current() {
        assert_eq!(BrightnessValue::DeltaRaw(1).resolve(u32::MAX, 100), 100);
    }

    #[test]
    fn resolves_positive_percent_delta() {
        assert_eq!(BrightnessValue::DeltaPercent(10.0).resolve(50, 100), 60);
    }

    #[test]
    fn resolves_negative_percent_delta() {
        assert_eq!(BrightnessValue::DeltaPercent(-10.0).resolve(50, 100), 40);
    }

    #[test]
    fn clamps_positive_percent_delta_to_max() {
        assert_eq!(BrightnessValue::DeltaPercent(1000.0).resolve(50, 100), 100);
    }

    #[test]
    fn clamps_negative_percent_delta_to_one_instead_of_zero() {
        assert_eq!(BrightnessValue::DeltaPercent(-1000.0).resolve(50, 100), 1);
    }

    #[test]
    fn no_op_delta_returns_current() {
        assert_eq!(BrightnessValue::DeltaRaw(0).resolve(50, 100), 50);
        assert_eq!(BrightnessValue::DeltaPercent(0.0).resolve(50, 100), 50);
    }

    #[test]
    fn resolves_delta_when_max_is_zero() {
        assert_eq!(BrightnessValue::DeltaRaw(5).resolve(0, 0), 0);
        // No floor of 1 applies when the device's max is itself 0 - 1
        // would be out of range.
        assert_eq!(BrightnessValue::DeltaPercent(50.0).resolve(0, 0), 0);
    }

    #[test]
    fn floors_zero_percent_to_one() {
        assert_eq!(BrightnessValue::Percent(0.0).resolve(0, 100), 1);
    }

    #[test]
    fn floors_tiny_percent_that_rounds_down_to_zero_to_one() {
        assert_eq!(BrightnessValue::Percent(0.1).resolve(0, 100), 1);
    }

    #[test]
    fn does_not_floor_percent_of_zero_max_device() {
        assert_eq!(BrightnessValue::Percent(50.0).resolve(0, 0), 0);
    }

    #[test]
    fn floors_percent_delta_that_rounds_down_to_zero_to_one() {
        assert_eq!(BrightnessValue::DeltaPercent(-50.0).resolve(50, 100), 1);
    }

    #[test]
    fn does_not_floor_percent_values_above_one() {
        assert_eq!(BrightnessValue::Percent(50.0).resolve(0, 100), 50);
        assert_eq!(BrightnessValue::DeltaPercent(-10.0).resolve(50, 100), 40);
    }
}
