//! Small shared helpers for widgets.

/// Clamp a progress fraction to `[0, 1]`, mapping non-finite input (NaN/∞) to `0.0`.
///
/// `f32::clamp` passes NaN through unchanged, so a NaN fraction would otherwise
/// survive and silently render nothing; widgets call this to honor their
/// documented `[0, 1]` contract.
pub(crate) fn sanitize_fraction(fraction: f32) -> f32 {
    if fraction.is_finite() {
        fraction.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_fraction;

    #[test]
    fn clamps_in_range() {
        assert_eq!(sanitize_fraction(-0.5), 0.0);
        assert_eq!(sanitize_fraction(1.5), 1.0);
        assert_eq!(sanitize_fraction(0.25), 0.25);
    }

    #[test]
    fn maps_non_finite_to_zero() {
        assert_eq!(sanitize_fraction(f32::NAN), 0.0);
        assert_eq!(sanitize_fraction(f32::INFINITY), 0.0);
        assert_eq!(sanitize_fraction(f32::NEG_INFINITY), 0.0);
    }
}
