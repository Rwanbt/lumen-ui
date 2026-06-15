//! Easing curves — named presets and arbitrary cubic-bézier, CSS-style.

/// An easing curve mapping normalized time `t ∈ [0, 1]` to eased progress `[0, 1]`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// Arbitrary cubic-bézier control points `(x1, y1, x2, y2)` (CSS semantics).
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    /// Evaluate the curve at `t` (clamped to `[0, 1]`).
    #[must_use]
    pub fn eval(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        let (x1, y1, x2, y2) = match self {
            Easing::Linear => return t,
            Easing::EaseIn => (0.42, 0.0, 1.0, 1.0),
            Easing::EaseOut => (0.0, 0.0, 0.58, 1.0),
            Easing::EaseInOut => (0.42, 0.0, 0.58, 1.0),
            Easing::CubicBezier(x1, y1, x2, y2) => (x1, y1, x2, y2),
        };
        cubic_bezier(t, x1, y1, x2, y2)
    }
}

// One axis of a cubic bézier with P0=0, P3=1 and the given control coordinates.
fn sample(c1: f32, c2: f32, u: f32) -> f32 {
    let a = 3.0 * c1;
    let b = 3.0 * c2 - 6.0 * c1;
    let c = 3.0 * c1 - 3.0 * c2 + 1.0;
    ((c * u + b) * u + a) * u
}

fn sample_derivative_x(x1: f32, x2: f32, u: f32) -> f32 {
    let a = 3.0 * x1;
    let b = 3.0 * x2 - 6.0 * x1;
    let c = 3.0 * x1 - 3.0 * x2 + 1.0;
    3.0 * c * u * u + 2.0 * b * u + a
}

/// Evaluate y for a given x on a CSS cubic-bézier timing curve.
fn cubic_bezier(x: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    // Find the parameter `u` such that sample_x(u) == x (Newton, then bisection fallback).
    let mut u = x;
    for _ in 0..8 {
        let dx = sample(x1, x2, u) - x;
        let d = sample_derivative_x(x1, x2, u);
        if d.abs() < 1e-6 {
            break;
        }
        u -= dx / d;
        u = u.clamp(0.0, 1.0);
    }
    // Bisection refine in case Newton stalled.
    let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
    for _ in 0..16 {
        let mid = sample(x1, x2, u);
        if (mid - x).abs() < 1e-4 {
            break;
        }
        if mid < x {
            lo = u;
        } else {
            hi = u;
        }
        u = 0.5 * (lo + hi);
    }
    sample(y1, y2, u)
}

#[cfg(test)]
mod tests {
    use super::Easing;

    #[test]
    fn endpoints_are_fixed() {
        for e in [
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::CubicBezier(0.25, 0.1, 0.25, 1.0),
        ] {
            assert!(e.eval(0.0).abs() < 1e-3, "{e:?} eval(0) != 0");
            assert!((e.eval(1.0) - 1.0).abs() < 1e-3, "{e:?} eval(1) != 1");
        }
    }

    #[test]
    fn linear_is_identity() {
        assert!((Easing::Linear.eval(0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn monotonic_nondecreasing() {
        let e = Easing::EaseInOut;
        let mut prev = -1.0;
        for i in 0..=20 {
            let v = e.eval(i as f32 / 20.0);
            assert!(v + 1e-3 >= prev, "easing should be non-decreasing");
            prev = v;
        }
    }
}
