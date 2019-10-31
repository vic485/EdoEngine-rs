pub mod vector;

const EPSILON: f32 = 0.00001;

/// Clamps a float value between [0.0, 1.0]
pub fn clamp01(value: f32) -> f32 {
    if value > 1.0 {
        1.0
    } else if value < 0.0 {
        0.0
    } else {
        value
    }
}
