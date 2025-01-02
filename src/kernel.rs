use glam::f32::Vec2;

use crate::consts::SMOOTHING_RADIUS;
use crate::random;

#[allow(dead_code)]
pub enum Kernel {
    Smooth6,
    Spiky2,
}

const SMOOTHING_RADIUS_INV: f32 = 1.0 / SMOOTHING_RADIUS;
const SMOOTHING_RADIUS_2: f32 = SMOOTHING_RADIUS * SMOOTHING_RADIUS;
#[allow(dead_code)]
const SMOOTHING_RADIUS_2_INV: f32 = 1.0 / SMOOTHING_RADIUS_2;

// Constants for smooth6 kernel.
#[allow(dead_code)]
pub const SMOOTH6_FACTOR: f32 =
    4.0 / (std::f32::consts::PI * SMOOTHING_RADIUS_2);
    #[allow(dead_code)]
pub const SMOOTH6_GRAD_FACTOR: f32 = 24.0 / (
    std::f32::consts::PI * SMOOTHING_RADIUS_2 * SMOOTHING_RADIUS_2
);

#[allow(dead_code)]
pub fn smooth6(displacement_squared: f32) -> f32 {
    if displacement_squared > SMOOTHING_RADIUS_2 {
        0.0
    } else {
        let value = 1.0 - displacement_squared * SMOOTHING_RADIUS_2_INV;
        SMOOTH6_FACTOR * value * value * value
    }
}

#[allow(dead_code)]
pub fn grad_smooth6(displacement: Vec2) -> Vec2 {
    let mag_2 = displacement.length_squared();
    if mag_2 > SMOOTHING_RADIUS_2 {
        Vec2::ZERO
    } else {
        if mag_2 < std::f32::EPSILON {
            let dir = random::vec_within_disk(1.0);
            SMOOTH6_GRAD_FACTOR * dir
        } else {
            let value = 1.0 - mag_2 * SMOOTHING_RADIUS_2_INV;
            SMOOTH6_GRAD_FACTOR * value * value * displacement
        }
    }
}

// Constants for spiky2 kernel.
#[allow(dead_code)]
pub const SPIKY2_FACTOR: f32 = 6.0 / (std::f32::consts::PI * SMOOTHING_RADIUS_2);
#[allow(dead_code)]
pub const SPIKY2_GRAD_FACTOR: f32 = 12.0 / (
    std::f32::consts::PI * SMOOTHING_RADIUS_2 * SMOOTHING_RADIUS
);

#[allow(dead_code)]
pub fn spiky2(displacement_squared: f32) -> f32 {
    if displacement_squared > SMOOTHING_RADIUS_2 {
        0.0
    } else {
        let distance = displacement_squared.sqrt();
        let value = 1.0 - distance * SMOOTHING_RADIUS_INV;
        SPIKY2_FACTOR * value * value
    }
}

#[allow(dead_code)]
pub fn grad_spiky2(displacement: Vec2) -> Vec2 {
    if displacement.length_squared() > SMOOTHING_RADIUS_2 {
        Vec2::ZERO
    } else {
        let distance = displacement.length();
        if distance < std::f32::EPSILON {
            // If we are at the centre of influence, pick a random direction.
            let dir = random::vec_within_disk(1.0);
            SPIKY2_GRAD_FACTOR * dir
        } else {
            SPIKY2_GRAD_FACTOR * (
                1.0 / distance - SMOOTHING_RADIUS_INV
            ) * displacement
        }
    }
}
