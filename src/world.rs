use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::{
    circle::CircleObject, geometry::Vector, object::MetaObject, rectangle::RectangleObject,
};

pub struct World {
    objects: Vec<MetaObject>,
}

impl World {
    pub fn new() -> Self {
        let mut objects = vec![
            RectangleObject::new_obj_immovable(0.0, 11.0, 10.0, 500.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(10.0, 0.0, 500.0, 10.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(512.0, 10.0, 10.0, 500.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(10.0, 512.0, 500.0, 10.0, Vector::new(0.0, 0.0)),
            // CircleObject::new_obj(Vector::new(100.0, 100.0), 15.0, Vector::new(0.0, 0.0)),
            CircleObject::new_obj(Vector::new(20.0, 20.0), 15.0, Vector::new(20.0, 20.0)),
            // RectangleObject::new_obj(100.0, 100.0, 30.0, 30.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj(100.0, 100.0, 30.0, 30.0, Vector::new(-20.0, -20.0)),
        ];
        let mut rng = rand::thread_rng();
        let rects = 0;
        let circles = 0;
        for _ in 0..rects {
            let size = rng.gen_range(5.0, 20.0);
            objects.push(RectangleObject::new_obj(
                rng.gen_range(2.0, 400.0),
                rng.gen_range(2.0, 400.0),
                size,
                size,
                Vector::new(rng.gen_range(-40.0, 40.0), rng.gen_range(-40.0, 40.0)),
            ))
        }
        for _ in 0..circles {
            let size = rng.gen_range(5.0, 30.0);
            let speed = Vector::new(rng.gen_range(20.0, 40.0), rng.gen_range(20.0, 40.0))
                * rng.gen_range(-1.0_f64, 1.0).signum();
            objects.push(CircleObject::new_obj(
                Vector::new(rng.gen_range(2.0, 400.0), rng.gen_range(2.0, 400.0)),
                size,
                speed,
            ))
        }
        Self { objects }
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        for obj in self.objects.iter() {
            obj.draw(context);
        }
    }

    pub fn tick(&mut self, delta_time: f64) {
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
                        self.objects[i].mov(vector * -1.0);

                        self.objects[j].kick(b_kick);
                        self.objects[j].mov(vector);
                    }
                }
            }
        }
    }
}
