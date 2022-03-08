use crate::collisions::{collide_circle_rect, collide_rect_rect};
use crate::geometry::{Rectangle, Vector};
use crate::object::MetaObject;

pub struct RectangleObject {
    pub shape: Rectangle,
    speed: Vector,
    immovable: bool,
}

impl RectangleObject {
    pub fn new(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> Self {
        Self {
            shape: Rectangle::new(x, y, w, h),
            speed,
            immovable: false,
        }
    }

    pub fn new_obj(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> MetaObject {
        MetaObject::Rect(Self::new(x, y, w, h, speed))
    }

    pub fn new_obj_immovable(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> MetaObject {
        MetaObject::Rect(RectangleObject {
            shape: Rectangle::new(x, y, w, h),
            speed,
            immovable: true,
        })
    }

    pub fn left(&self) -> f64 {
        self.shape.left()
    }

    pub fn right(&self) -> f64 {
        self.shape.right()
    }

    pub fn top(&self) -> f64 {
        self.shape.top()
    }

    pub fn bottom(&self) -> f64 {
        self.shape.bottom()
    }

    pub fn center(&self) -> Vector {
        self.shape.center()
    }
}

impl RectangleObject {
    pub fn tick(&mut self, delta_time: f64) {
        if self.immovable {
            return;
        }
        self.shape.coord += self.speed * delta_time;
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match other {
            MetaObject::Rect(rect) => collide_rect_rect(self, rect),
            MetaObject::Circle(circle) => collide_circle_rect(circle, self).map(|x| x * -1.0),
        }
    }

    pub fn mov(&mut self, direction: Vector) {
        if self.immovable {
            return;
        }
        self.shape.coord += direction;
    }

    pub fn kick(&mut self, speed: Vector) {
        if self.immovable {
            return;
        }
        self.speed += speed;
    }

    pub fn speed(&self) -> Vector {
        self.speed
    }

    pub fn aabb(&self) -> Rectangle {
        self.shape
    }
}
