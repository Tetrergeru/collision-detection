use rand::Rng;

use crate::{
    circle::CircleObject, geometry::Vector, object::MetaObject, rectangle::RectangleObject,
};

pub struct World {
    objects: Vec<MetaObject>,
}

impl World {
    pub fn new() -> Self {
        let speed_rng = (10.0, 20.1);

        let mut objects = vec![
            RectangleObject::new_obj_immovable(0.0, 11.0, 10.0, 500.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(10.0, 0.0, 500.0, 10.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(512.0, 10.0, 10.0, 500.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(10.0, 512.0, 500.0, 10.0, Vector::new(0.0, 0.0)),
            // CircleObject::new_obj(Vector::new(100.0, 100.0), 15.0, Vector::new(0.0, 0.0)),
            // CircleObject::new_obj(Vector::new(20.0, 20.0), 15.0, Vector::new(20.0, 20.0)),
            // RectangleObject::new_obj(100.0, 100.0, 30.0, 30.0, Vector::new(0.0, 0.0)),
            // RectangleObject::new_obj(100.0, 100.0, 30.0, 30.0, Vector::new(-20.0, -20.0)),
        ];
        let mut rng = rand::thread_rng();
        let rects = 1000;
        let circles = 1000;
        for _ in 0..rects {
            let size = rng.gen_range(3.0, 6.0);
            let speed = Vector::new(
                rng.gen_range(speed_rng.0, speed_rng.1),
                rng.gen_range(speed_rng.0, speed_rng.1),
            ) * rng.gen_range(-1.0_f64, 1.0).signum();

            objects.push(RectangleObject::new_obj(
                rng.gen_range(2.0, 400.0),
                rng.gen_range(2.0, 400.0),
                size,
                size,
                speed,
            ))
        }
        for _ in 0..circles {
            let size = rng.gen_range(1.5, 3.0);
            let speed = Vector::new(
                rng.gen_range(speed_rng.0, speed_rng.1),
                rng.gen_range(speed_rng.0, speed_rng.1),
            ) * rng.gen_range(-1.0_f64, 1.0).signum();

            objects.push(CircleObject::new_obj(
                Vector::new(rng.gen_range(2.0, 400.0), rng.gen_range(2.0, 400.0)),
                size,
                speed,
            ))
        }
        Self { objects }
    }

    pub fn export(&self) -> Box<[f64]> {
        let mut vec = Vec::with_capacity(self.objects.len() * 4);

        for object in self.objects.iter() {
            match object {
                MetaObject::Rect(rect) => {
                    vec.push(1.0);
                    vec.push(rect.left());
                    vec.push(rect.top());
                    vec.push(rect.shape.size.x);
                    vec.push(rect.shape.size.y);
                }
                MetaObject::Circle(circle) => {
                    vec.push(2.0);
                    vec.push(circle.center.x);
                    vec.push(circle.center.y);
                    vec.push(circle.radius);
                }
            }
        }

        vec.into_boxed_slice()
    }

    pub fn tick(&mut self, delta_time: f64) {
        // return;
        for obj in self.objects.iter_mut() {
            obj.tick(delta_time);
        }
        for i in 0..self.objects.len() {
            for j in (i + 1)..self.objects.len() {
                let a = &self.objects[i];
                let b = &self.objects[j];
                if !a.aabb().collides_with(&b.aabb()) {
                    // log::info!("{}, {} -> {:?} {:?}", i, j, a.aabb(), b.aabb());
                    continue;
                }

                let collision = a.collides_with(b);

                if let Some(vector) = collision {
                    let vector_norm = vector.norm();
                    if vector.len().abs() > 0.00000001 {
                        let a_collision_speed = vector_norm.dot(&a.speed());
                        let b_collision_speed = vector_norm.dot(&b.speed());

                        let a_kick = vector_norm * (-2.0 * a_collision_speed);
                        let b_kick = vector_norm * (-2.0 * b_collision_speed);

                        self.objects[i].kick(a_kick);
                        self.objects[i].mov(vector * -0.5);

                        self.objects[j].kick(b_kick);
                        self.objects[j].mov(vector * 0.5);
                    }
                }
            }
        }
    }
}
