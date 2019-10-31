use edocore::math::vector::Vector3;

#[test]
fn test_constructor() {
    let a = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let b = Vector3::new(1.0, 2.0, 3.0);

    assert_eq!(a.x, b.x);
    assert_eq!(a.y, b.y);
    assert_eq!(a.z, b.z);
}

#[test]
fn test_cross() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(4.0, 5.0, 6.0);
    let c = Vector3::cross(a, b);

    assert_eq!(c.x, -3.0);
    assert_eq!(c.y, 6.0);
    assert_eq!(c.z, -3.0);
}

#[test]
fn test_dot() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(4.0, 5.0, 6.0);

    assert_eq!(Vector3::dot(a, b), 32.0);
}
