use crate::geometry::{one_dimensional_collision, Rectangle, Vector};
use crate::object::MetaObject;

use web_sys::CanvasRenderingContext2d;

pub struct RectangleObject {
    shape: Rectangle,
    speed: Vector,
    immovable: bool,
}

impl RectangleObject {
    pub fn new_obj(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> MetaObject {
        MetaObject::Rect(RectangleObject {
            shape: Rectangle { x, y, w, h },
            speed,
            immovable: false,
        })
    }

    pub fn new_obj_immovable(x: f64, y: f64, w: f64, h: f64, speed: Vector) -> MetaObject {
        MetaObject::Rect(RectangleObject {
            shape: Rectangle { x, y, w, h },
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
}

impl RectangleObject {
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        context.save();
        context.set_fill_style(&("#f00".to_string()).into());
        context.begin_path();
        context.move_to(self.left(), self.top());
        context.line_to(self.right(), self.top());
        context.line_to(self.right(), self.bottom());
        context.line_to(self.left(), self.bottom());
        context.close_path();
        context.fill();
        context.restore();
    }

    pub fn tick(&mut self, delta_time: f64) {
        if self.immovable {
            return;
        }
        self.shape.x += self.speed.x() * delta_time;
        self.shape.y += self.speed.y() * delta_time;
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match other {
            MetaObject::Rect(rect) => {
                let x = -one_dimensional_collision(
                    Vector::new(self.left(), self.right()),
                    Vector::new(rect.left(), rect.right()),
                );
                let y = -one_dimensional_collision(
                    Vector::new(self.top(), self.bottom()),
                    Vector::new(rect.top(), rect.bottom()),
                );
                if x.is_nan() || y.is_nan() {
                    return None;
                }
                if x.abs() > y.abs() {
                    Some(Vector::new(x, 0.0))
                } else {
                    Some(Vector::new(0.0, y))
                }
            }
            _ => None,
        }
    }

    pub fn kick(&mut self, speed: Vector) {
        if self.immovable {
            return;
        }
        self.speed.x += speed.x();
        self.speed.y += speed.y();
    }

    pub fn speed(&self) -> Vector {
        self.speed
    }
}
