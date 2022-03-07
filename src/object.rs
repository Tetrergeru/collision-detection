use crate::{circle::CircleObject, geometry::{Vector, Rectangle}, rectangle::RectangleObject};
use web_sys::CanvasRenderingContext2d;

pub enum MetaObject {
    Rect(RectangleObject),
    Circle(CircleObject),
}

impl MetaObject {
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        match self {
            MetaObject::Rect(rect) => rect.draw(context),
            MetaObject::Circle(circle) => circle.draw(context),
        }
    }

    pub fn tick(&mut self, delta_time: f64) {
        match self {
            MetaObject::Rect(rect) => rect.tick(delta_time),
            MetaObject::Circle(circle) => circle.tick(delta_time),
        }
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match self {
            MetaObject::Rect(rect) => rect.collides_with(other),
            MetaObject::Circle(circle) => circle.collides_with(other),
        }
    }

    pub fn speed(&self) -> Vector {
        match self {
            MetaObject::Rect(rect) => rect.speed(),
            MetaObject::Circle(circle) => circle.speed(),
        }
    }

    pub fn mov(&mut self, direction: Vector) {
        match self {
            MetaObject::Rect(rect) => rect.mov(direction),
            MetaObject::Circle(circle) => circle.mov(direction),
        }
    }

    pub fn kick(&mut self, speed: Vector) {
        match self {
            MetaObject::Rect(rect) => rect.kick(speed),
            MetaObject::Circle(circle) => circle.kick(speed),
        }
    }

    pub fn aabb(&self) -> Rectangle {
        match self {
            MetaObject::Rect(rect) => rect.aabb(),
            MetaObject::Circle(circle) => circle.aabb(),
        }
    }
}
