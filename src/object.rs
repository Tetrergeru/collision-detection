use crate::{
    circle::CircleObject,
    geometry::{Rectangle, Vector},
    rectangle::RectangleObject, polyhedron::PolyhedronObject,
};

pub enum MetaObject {
    Rect(RectangleObject),
    Circle(CircleObject),
    Poly(PolyhedronObject),
}

impl MetaObject {
    pub fn tick(&mut self, delta_time: f64) {
        match self {
            MetaObject::Rect(rect) => rect.tick(delta_time),
            MetaObject::Circle(circle) => circle.tick(delta_time),
            MetaObject::Poly(poly) => poly.tick(delta_time),
        }
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match self {
            MetaObject::Rect(rect) => rect.collides_with(other),
            MetaObject::Circle(circle) => circle.collides_with(other),
            MetaObject::Poly(poly) => poly.collides_with(other),
        }
    }

    pub fn speed(&self) -> Vector {
        match self {
            MetaObject::Rect(rect) => rect.speed(),
            MetaObject::Circle(circle) => circle.speed(),
            MetaObject::Poly(poly) => poly.speed(),
        }
    }

    pub fn mov(&mut self, direction: Vector) {
        match self {
            MetaObject::Rect(rect) => rect.mov(direction),
            MetaObject::Circle(circle) => circle.mov(direction),
            MetaObject::Poly(poly) => poly.mov(direction),
        }
    }

    pub fn kick(&mut self, speed: Vector) {
        match self {
            MetaObject::Rect(rect) => rect.kick(speed),
            MetaObject::Circle(circle) => circle.kick(speed),
            MetaObject::Poly(poly) => poly.kick(speed),
        }
    }

    pub fn aabb(&self) -> Rectangle {
        match self {
            MetaObject::Rect(rect) => rect.aabb(),
            MetaObject::Circle(circle) => circle.aabb(),
            MetaObject::Poly(poly) => poly.aabb(),
        }
    }
}
