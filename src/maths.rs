pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + f32::exp(-x))
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
