use crate::{
    circle::CircleObject,
    collisions::collide_circle_rect,
    geometry::{project_point, Rectangle, Vector},
    rectangle::RectangleObject,
};

#[test]
fn test_point_projection_on_vertical() {
    let line = Rectangle::new(0.0, 0.0, 0.0, 1.0);
    let point = Vector::new(2.0, 2.0);
    assert!((project_point(&line, point) - 2.0).abs() < 0.000001);
}

#[test]
fn test_point_projection_on_horisontal() {
    let line = Rectangle::new(0.0, 0.0, 1.0, 0.0);
    let point = Vector::new(2.0, 2.0);
    assert!((project_point(&line, point) - 2.0).abs() < 0.000001);
}

#[test]
fn test_point_projection_on_diagonal() {
    let line = Rectangle::new_vec(Vector::new(0.0, 0.0), Vector::new(1.0, 1.0).norm());
    let point = Vector::new(1.0, 1.0);
    assert!((project_point(&line, point) - Vector::new(1.0, 1.0).len()).abs() < 0.000001);
}

#[test]
fn test_point_projection_on_non_zero() {
    let line = Rectangle::new_vec(Vector::new(1.0, 1.0), Vector::new(1.0, 1.0).norm());
    let point = Vector::new(2.0, 2.0);
    assert!((project_point(&line, point) - Vector::new(1.0, 1.0).len()).abs() < 0.000001);
}

#[test]
fn test_circle_rect_collision() {
    let circle = CircleObject::new(Vector::new(50.0, 50.0), 30.0, Vector::zero());
    let rect = RectangleObject::new(10.0, 10.0, 100.0, 30.0, Vector::zero());

    if let Some(collision) = collide_circle_rect(&circle, &rect) {
        assert!((collision - Vector::new(0.0, 20.0)).len() < 0.0000001);
    } else {
        panic!("Expected to find collision");
    }
}
