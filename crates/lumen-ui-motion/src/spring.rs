//! Spring physics — frame-rate-independent value animation.
//!
//! State (value + velocity) is kept in `ctx.data` keyed by an id, integrated each
//! frame with the frame's delta time. While a spring is still moving, a repaint is
//! requested so it keeps ticking.

use egui::{Color32, Context, Id};

const SETTLE_VALUE_EPS: f32 = 0.001;
const SETTLE_VELOCITY_EPS: f32 = 0.001;
const MAX_DT: f32 = 1.0 / 30.0; // clamp large frames for stability

#[derive(Clone, Copy, Default)]
struct SpringState {
    value: f32,
    velocity: f32,
    initialized: bool,
}

/// A critically-tunable spring. Lower `damping` = more overshoot/wobble.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self::SMOOTH
    }
}

impl Spring {
    /// Snappy, no overshoot.
    pub const SMOOTH: Spring = Spring {
        stiffness: 170.0,
        damping: 26.0,
        mass: 1.0,
    };
    /// Soft and slow.
    pub const GENTLE: Spring = Spring {
        stiffness: 120.0,
        damping: 20.0,
        mass: 1.0,
    };
    /// Bouncy overshoot.
    pub const WOBBLY: Spring = Spring {
        stiffness: 180.0,
        damping: 12.0,
        mass: 1.0,
    };
    /// Fast and tight.
    pub const STIFF: Spring = Spring {
        stiffness: 260.0,
        damping: 30.0,
        mass: 1.0,
    };

    #[must_use]
    pub fn new(stiffness: f32, damping: f32, mass: f32) -> Self {
        Self {
            stiffness,
            damping,
            mass: mass.max(f32::EPSILON),
        }
    }

    /// Advance the spring towards `target` and return the current value.
    ///
    /// `id` must be stable across frames for a given animated quantity.
    pub fn animate(&self, ctx: &Context, id: Id, target: f32) -> f32 {
        let dt = ctx.input(|i| i.stable_dt).min(MAX_DT);
        let mut state: SpringState = ctx.data_mut(|d| d.get_temp(id).unwrap_or_default());

        if !state.initialized {
            state = SpringState {
                value: target,
                velocity: 0.0,
                initialized: true,
            };
        } else if dt > 0.0 {
            // Semi-implicit Euler: a = (-k·x - c·v) / m
            let force = -self.stiffness * (state.value - target) - self.damping * state.velocity;
            let accel = force / self.mass;
            state.velocity += accel * dt;
            state.value += state.velocity * dt;
        }

        let settled = (state.value - target).abs() < SETTLE_VALUE_EPS
            && state.velocity.abs() < SETTLE_VELOCITY_EPS;
        if settled {
            state.value = target;
            state.velocity = 0.0;
        } else {
            ctx.request_repaint();
        }

        ctx.data_mut(|d| d.insert_temp(id, state));
        state.value
    }

    /// Animate each channel of a color with this spring.
    pub fn animate_color(&self, ctx: &Context, id: Id, target: Color32) -> Color32 {
        let chan = |value: u8, channel: u8| -> u8 {
            self.animate(ctx, id.with(channel), f32::from(value))
                .round()
                .clamp(0.0, 255.0) as u8
        };
        Color32::from_rgba_unmultiplied(
            chan(target.r(), 0),
            chan(target.g(), 1),
            chan(target.b(), 2),
            chan(target.a(), 3),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Spring;

    #[test]
    fn presets_are_positive_and_stable() {
        for s in [
            Spring::SMOOTH,
            Spring::GENTLE,
            Spring::WOBBLY,
            Spring::STIFF,
        ] {
            assert!(s.stiffness > 0.0 && s.damping > 0.0 && s.mass > 0.0);
        }
    }

    #[test]
    fn new_guards_against_zero_mass() {
        assert!(Spring::new(100.0, 10.0, 0.0).mass > 0.0);
    }
}
