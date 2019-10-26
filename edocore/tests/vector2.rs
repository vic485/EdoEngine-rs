use edocore::vector2::Vector2;

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
