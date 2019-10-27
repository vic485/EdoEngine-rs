use edocore::math::vector2::Vector2;

#[test]
fn test_constructor() {
    let a = Vector2{x: 2.0, y: 3.0};
    let b = Vector2::new(2.0, 3.0);
    
    assert_eq!(a.x, b.x);
    assert_eq!(a.y, b.y)
}

#[test]
fn test_square_magnitude() {
    assert_eq!(Vector2::new(3.0, 4.0).square_magnitude(), 25.0)
}

#[test]
fn test_dot() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(3.0, 4.0);
    
    assert_eq!(Vector2::dot(a, b), 11.0)
}

#[test]
fn test_add() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(3.0, 4.0);
    let c = a + b;
    let result = Vector2::new(4.0, 6.0);
    
    assert_eq!(c.x, result.x);
    assert_eq!(c.y, result.y);
}
