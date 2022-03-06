use crate::{geometry::Vector, rectangle::RectangleObject};
use web_sys::CanvasRenderingContext2d;

pub enum MetaObject {
    Rect(RectangleObject),
}

impl MetaObject {
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        match self {
            MetaObject::Rect(rect) => rect.draw(context),
        }
    }

    pub fn tick(&mut self, delta_time: f64) {
        match self {
            MetaObject::Rect(rect) => rect.tick(delta_time),
        }
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match self {
            MetaObject::Rect(rect) => rect.collides_with(other),
        }
    }

    pub fn speed(&self) -> Vector {
        match self {
            MetaObject::Rect(rect) => rect.speed(),
        }
    }

    pub fn kick(&mut self, speed: Vector) {
        match self {
            MetaObject::Rect(rect) => rect.kick(speed),
        }
    }
}
