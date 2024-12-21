use glam::f32::Vec2;
use rand::Rng;

pub fn point_in_box(extent: (f32, f32)) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    (
        rng.gen_range(-extent.0..extent.0),
        rng.gen_range(-extent.1..extent.1),
    )
}

pub fn vec_within_disk(max_radius: f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    let v_mag: f32 = rng.gen_range(0.0..max_radius);
    let z: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let v_x = v_mag * f32::cos(z);
    let v_y = v_mag * f32::sin(z);
    Vec2{
        x: v_x,
        y: v_y,
    }
}
