/// Smoothly ramps x values from 0 to 1 over the linear interval [0, 1].
pub fn smooth_ramp(x: f32) -> f32 {
    1.0_f32.min(x * x)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
