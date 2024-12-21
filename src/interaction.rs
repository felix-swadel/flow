use bevy::prelude::*;

use crate::color;
use crate::consts::PARTICLE_MAX_INITIAL_V;
use crate::consts_private::BOX_HALF_SIZE;
use crate::particle::ParticleVelocity;
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
        let (x, y) = random::point_in_box(BOX_HALF_SIZE);
        let v = random::vec_within_disk(PARTICLE_MAX_INITIAL_V);
        *transform = Transform::from_xyz(x, y, 1.0);
        velocity.0 = v;
        materials.get_mut(color_handle).unwrap().color =
            color::for_velocity(v.length());
    }
}
