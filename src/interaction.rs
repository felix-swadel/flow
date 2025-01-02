use bevy::prelude::*;

use crate::color;
use crate::consts_private::SCREEN_FACTOR;
use crate::particle::{
    ParticleAcceleration,
    ParticleDensity,
    ParticlePosition,
    ParticlePressure,
    ParticleVelocity,
    PrevParticlePosition,
    PHYSICAL_HALF_SIZE
};
use crate::random;

pub fn keypress(
    mut query: Query<(
        &mut ParticleDensity,
        &mut ParticlePressure,
        &mut PrevParticlePosition,
        &mut ParticlePosition,
        &mut ParticleVelocity,
        &mut ParticleAcceleration,
        &mut Transform,
        &MeshMaterial2d<ColorMaterial>
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Re-spawn particles on spacebar.
    for (
        mut density,
        mut pressure,
        mut prev_position,
        mut position,
        mut velocity,
        mut acceleration,
        mut transform,
        color_handle,
    ) in &mut query {
        density.0 = 0.0;
        pressure.0 = 0.0;
        prev_position.0 = None;
        let (x, y) = random::point_in_box(PHYSICAL_HALF_SIZE);
        position.0 = Vec2{ x, y };
        velocity.0 = Vec2::ZERO;
        acceleration.0 = Vec2::ZERO;
        transform.translation.x = x * SCREEN_FACTOR;
        transform.translation.y = y * SCREEN_FACTOR;
        materials.get_mut(color_handle).unwrap().color =
            color::for_velocity(0.0);
    }
}
