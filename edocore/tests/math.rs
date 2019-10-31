use edocore::math::clamp01;

#[test]
fn test_clamp01() {
    assert_eq!(clamp01(2.0), 1.0);
    assert_eq!(clamp01(-3.0), 0.0);
    assert_eq!(clamp01(0.7), 0.7);
}
