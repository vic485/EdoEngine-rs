pub mod vector;
pub mod matrix;

const EPSILON: f32 = 0.00001;

/// Clamps a float value between [min, max]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    }
}

/// Clamps a float value between [0.0, 1.0]
pub fn clamp01(value: f32) -> f32 {
    clamp(value, 0.0, 1.0)
}
