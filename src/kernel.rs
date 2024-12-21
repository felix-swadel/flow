use glam::f32::Vec2;

use crate::consts::SMOOTHING_RADIUS;

const SMOOTHING_RADIUS_2: f32 = SMOOTHING_RADIUS * SMOOTHING_RADIUS;
const SMOOTHING_RADIUS_2_INV: f32 = 1.0 / SMOOTHING_RADIUS_2;
// The volume of (4 / pi * s^2) (1 - (x/s)^2)^3 is 1.
pub const MAX_VAL: f32 =
    4.0 / (std::f32::consts::PI * SMOOTHING_RADIUS_2);
pub const GRAD_SCALE: f32 = 24.0 / (
    std::f32::consts::PI * SMOOTHING_RADIUS_2 * SMOOTHING_RADIUS_2
);

pub fn compute(displacement: Vec2) -> f32 {
    let mag_2 = displacement.length();
    if mag_2 > SMOOTHING_RADIUS_2 {
        0.0
    } else {
        let value = 1.0 - mag_2 * SMOOTHING_RADIUS_2_INV;
        MAX_VAL * value * value * value
    }
}

pub fn gradient(displacement: Vec2) -> Vec2 {
    let mag_2 = displacement.length_squared();
    if mag_2 > SMOOTHING_RADIUS_2 {
        Vec2::ZERO
    } else {
        let value = 1.0 - mag_2 * SMOOTHING_RADIUS_2_INV;
        GRAD_SCALE * value * value * displacement
    }
}