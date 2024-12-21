use bevy::prelude::*;

use crate::color;
use crate::particle::ParticlePosition;
use crate::consts::PIXEL_SIZE;
use crate::consts_private::{BOX_HALF_SIZE, SCREEN_FACTOR};

#[derive(Component)]
pub struct Background;

// Convenience constants.
const PIXEL_SIZE_F: f32 = PIXEL_SIZE as f32;
const SCREEN_FACTOR_INV: f32 = 1.0 / SCREEN_FACTOR;

fn idx_to_screen_space(x: u32, y: u32) -> Vec2 {
    Vec2 {
        x: -BOX_HALF_SIZE.0 + (x as f32 + 0.5) * PIXEL_SIZE_F,
        y: BOX_HALF_SIZE.1 - (y as f32 + 0.5) * PIXEL_SIZE_F,
    } * SCREEN_FACTOR_INV
}

pub fn update(
    particles: Query<&ParticlePosition>,
    sprite: Query<&mut Sprite, With<Background>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.get_mut(&sprite.single().image).unwrap();
    let size = image.size();
    for i in 0..size.x {
        for j in 0..size.y {
            let sample_point = idx_to_screen_space(i, j);
            image.set_color_at(
                i,
                j,
                color::for_density(sample_point, particles.iter()),
            ).unwrap();
        }
    }
}
