use edocore::math::{clamp, clamp01};

#[test]
fn test_clamp() {
    assert_eq!(clamp(3.75, 1.0, 3.2), 3.2);
    assert_eq!(clamp(-27.0, 3.0, 5.7), 3.0);
    assert_eq!(clamp(2.7, 1.0, 3.0), 2.7);
}

#[test]
fn test_clamp01() {
    assert_eq!(clamp01(2.0), 1.0);
    assert_eq!(clamp01(-3.0), 0.0);
    assert_eq!(clamp01(0.7), 0.7);
}
