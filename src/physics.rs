use crate::consts::{PRESSURE_MULTIPLIER, TARGET_DENSITY};

/// Performs a 1D integration of a position [x](f32) moving at a velocity [v](f32)
/// moving in a bounded range from [low](f32) to [high](f32) with velocity damping
/// on collisions.
/// 
/// [x](f32) and [v](f32) are updated in place.
pub fn integrate_velocity(
    x: &mut f32, v: &mut f32, dt: f32,
    low: f32, high: f32, collision_damping: f32,
) {
    let new_x = *x + *v * dt;
    if new_x < low {
        *v *= -collision_damping;
        *x = 2.0 * low - new_x;
    } else if high < new_x {
        *v *= -collision_damping;
        *x = 2.0 * high - new_x;
    } else {
        *x = new_x;
    }
}

pub fn density_to_pressure(density: f32) -> f32 {
    let density_error = density - TARGET_DENSITY;
    density_error * PRESSURE_MULTIPLIER
}
