use bevy::prelude::*;

use crate::color;
use crate::consts::PARTICLE_MAX_INITIAL_V;
use crate::consts_private::SCREEN_FACTOR;
use crate::particle::{ParticleVelocity, PHYSICAL_HALF_SIZE};
use crate::random;

pub fn keypress(
    mut query: Query<(
        &mut Transform,
        &mut ParticleVelocity,
        &MeshMaterial2d<ColorMaterial>
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Re-spawn particles on spacebar.
    for (
        mut transform,
        mut velocity,
        color_handle,
    ) in &mut query {
        let (x, y) = random::point_in_box(PHYSICAL_HALF_SIZE);
        let v = random::vec_within_disk(PARTICLE_MAX_INITIAL_V);
        transform.translation.x += x * SCREEN_FACTOR;
        transform.translation.y += y * SCREEN_FACTOR;
        velocity.0 = v;
        materials.get_mut(color_handle).unwrap().color =
            color::for_velocity(v.length());
    }
}
