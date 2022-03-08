use std::iter;

use crate::{
    circle::CircleObject,
    geometry::{project_circle, project_point, project_points, Rectangle, Vector},
    polyhedron::PolyhedronObject,
    rectangle::RectangleObject,
};

pub fn collide_rect_rect(a: &RectangleObject, b: &RectangleObject) -> Option<Vector> {
    collide_aabb_aabb(&a.aabb(), &b.aabb())
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

pub fn collide_poly_rect(a: &PolyhedronObject, b: &RectangleObject) -> Option<Vector> {
    let mut min = f64::MAX;
    let mut res = None;

    let rect_line_x = Rectangle::new_vec(b.shape.coord, Vector::new(1.0, 0.0));
    let rect_line_y = Rectangle::new_vec(b.shape.coord, Vector::new(0.0, 1.0));

    for line in a.lines().chain(iter::once(rect_line_x)).chain(iter::once(rect_line_y)) {
        let a_proj = project_points(&line, a.points());
        let b_proj = project_points(&line, b.shape.points());

        let c = one_dimensional_collision(b_proj, a_proj);

        if c.is_nan() {
            return None;
        }
        if c.abs() < min {
            min = c.abs();
            res = Some(line.size * c);
        }
    }
    res
}

pub fn collide_poly_circle(a: &PolyhedronObject, b: &CircleObject) -> Option<Vector> {
    let mut min = f64::MAX;
    let mut res = None;
    let circle_line = Rectangle::new_vec(a.center, (b.center - a.center).norm());

    for line in a.lines().chain(iter::once(circle_line)) {
        let a_proj = project_points(&line, a.points());
        let b_proj = project_circle(&line, b.center, b.radius);

        let c = one_dimensional_collision(b_proj, a_proj);
        if c.is_nan() {
            return None;
        }
        if c.abs() < min {
            min = c.abs();
            res = Some(line.size * c);
        }
    }
    res
}

pub fn collide_poly_poly(a: &PolyhedronObject, b: &PolyhedronObject) -> Option<Vector> {
    let mut min = f64::MAX;
    let mut res = None;

    for line in a.lines().chain(b.lines()) {
        let a_proj = project_points(&line, a.points());
        let b_proj = project_points(&line, b.points());

        let c = one_dimensional_collision(b_proj, a_proj);
        if c.is_nan() {
            return None;
        }
        if c.abs() < min {
            min = c.abs();
            res = Some(line.size * c);
        }
    }
    res
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

pub fn collide_aabb_aabb(a: &Rectangle, b: &Rectangle) -> Option<Vector> {
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
