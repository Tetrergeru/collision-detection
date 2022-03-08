use crate::{
    collisions::{collide_circle_rect, collide_cirle_circle},
    geometry::{Rectangle, Vector},
    object::MetaObject,
};

pub struct CircleObject {
    pub center: Vector,
    pub radius: f64,
    pub speed: Vector,
}

impl CircleObject {
    pub fn new(center: Vector, radius: f64, speed: Vector) -> Self {
        Self {
            center,
            radius,
            speed,
        }
    }

    pub fn new_obj(center: Vector, radius: f64, speed: Vector) -> MetaObject {
        MetaObject::Circle(Self::new(center, radius, speed))
    }

    pub fn tick(&mut self, delta_time: f64) {
        self.center += self.speed() * delta_time;
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match other {
            MetaObject::Circle(circle) => collide_cirle_circle(self, circle),
            MetaObject::Rect(rect) => collide_circle_rect(self, rect),
        }
    }

    pub fn speed(&self) -> Vector {
        self.speed
    }

    pub fn mov(&mut self, direction: Vector) {
        self.center += direction;
    }

    pub fn kick(&mut self, speed: Vector) {
        self.speed += speed
    }

    pub fn aabb(&self) -> Rectangle {
        let rad = Vector::new(self.radius, self.radius);
        Rectangle::new_vec(self.center - rad, rad * 2.0)
    }
}
