use crate::{
    circle::CircleObject,
    geometry::{project_point, project_points, Rectangle, Vector},
    rectangle::RectangleObject,
};

pub fn collide_rect_rect(a: &RectangleObject, b: &RectangleObject) -> Option<Vector> {
    let x = one_dimensional_collision(
        Vector::new(b.left(), b.right()),
        Vector::new(a.left(), a.right()),
    );
    let y = one_dimensional_collision(
        Vector::new(b.top(), b.bottom()),
        Vector::new(a.top(), a.bottom()),
    );
    if x.is_nan() || y.is_nan() {
        return None;
    }
    if x.abs() < y.abs() {
        Some(Vector::new(x, 0.0))
    } else {
        Some(Vector::new(0.0, y))
    }
}

pub fn collide_cirle_circle(a: &CircleObject, b: &CircleObject) -> Option<Vector> {
    let d = (a.center - b.center).len();
    let shift_len = a.radius + b.radius - d;
    if shift_len > 0.0 {
        return Some((b.center - a.center) * (shift_len / d));
    }
    None
}

pub fn collide_circle_rect(a: &CircleObject, b: &RectangleObject) -> Option<Vector> {
    let x = one_dimensional_collision(
        Vector::new(b.left(), b.right()),
        Vector::new(a.center.x - a.radius, a.center.x + a.radius),
    );

    let y = one_dimensional_collision(
        Vector::new(b.top(), b.bottom()),
        Vector::new(a.center.y - a.radius, a.center.y + a.radius),
    );

    let p_line = Rectangle::new_vec(a.center, (a.center - b.center()).norm());
    let projected_circle = project_point(&p_line, a.center);
    let p = one_dimensional_collision(
        project_points(&p_line, b.shape.points()),
        Vector::new(projected_circle - a.radius, projected_circle + a.radius),
    );

    if x.is_nan() || y.is_nan() || p.is_nan() {
        return None;
    }

    let some = if x.abs() < y.abs() {
        if x.abs() < p.abs() {
            Vector::new(x, 0.0)
        } else {
            p_line.size * p
        }
    } else if y.abs() < p.abs() {
        Vector::new(0.0, y)
    } else {
        p_line.size * p
    };

    Some(some)
}

pub fn one_dimensional_collision(a: Vector, b: Vector) -> f64 {
    println!("{:?} {:?}", a, b);
    if a.y < b.x || a.x > b.y {
        return f64::NAN;
    }
    if a.y < b.y && a.x < b.x {
        return b.x - a.y;
    }
    if a.y > b.y && a.x > b.x {
        return b.y - a.x;
    }
    f64::MAX
}
