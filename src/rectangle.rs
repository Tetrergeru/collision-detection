use crate::collisions::{collide_circle_rect, collide_poly_rect, collide_rect_rect};
use crate::geometry::{Rectangle, Vector};
use crate::object::MetaObject;

pub struct RectangleObject {
    pub shape: Rectangle,
    speed: Vector,
}

impl RectangleObject {
    pub fn new(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> Self {
        Self {
            shape: Rectangle::new(x, y, w, h),
            speed,
        }
    }

    pub fn new_obj(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> MetaObject {
        MetaObject::Rect(Self::new(x, y, w, h, speed))
    }

    pub fn left(&self) -> f64 {
        self.shape.left()
    }

    pub fn top(&self) -> f64 {
        self.shape.top()
    }

    pub fn center(&self) -> Vector {
        self.shape.center()
    }
}

impl RectangleObject {
    pub fn tick(&mut self, delta_time: f64) {
        self.shape.coord += self.speed * delta_time;
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match other {
            MetaObject::Rect(rect) => collide_rect_rect(self, rect),
            MetaObject::Circle(circle) => collide_circle_rect(circle, self).map(|x| x * -1.0),
            MetaObject::Poly(poly) => collide_poly_rect(poly, self).map(|x| x * -1.0),
        }
    }

    pub fn mov(&mut self, direction: Vector) {
        self.shape.coord += direction;
    }

    pub fn kick(&mut self, speed: Vector) {
        self.speed += speed;
    }

    pub fn speed(&self) -> Vector {
        self.speed
    }

    pub fn aabb(&self) -> Rectangle {
        self.shape
    }
}
