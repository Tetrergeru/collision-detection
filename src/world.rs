use rand::Rng;
use web_sys::CanvasRenderingContext2d;

use crate::{
    geometry::{Rectangle, Vector},
    object::MetaObject,
    rectangle::RectangleObject,
};

pub struct World {
    objects: Vec<MetaObject>,
}

impl World {
    pub fn new() -> Self {
        let mut objects = vec![
            RectangleObject::new_obj_immovable(0.0, 10.0, 10.0, 500.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(10.0, 0.0, 500.0, 10.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(510.0, 10.0, 10.0, 500.0, Vector::new(0.0, 0.0)),
            RectangleObject::new_obj_immovable(10.0, 510.0, 500.0, 10.0, Vector::new(0.0, 0.0)),
        ];
        let mut rng = rand::thread_rng();
        for i in 0..20 {
            let size = rng.gen_range(2.0, 40.0);
            objects.push(RectangleObject::new_obj(
                rng.gen_range(2.0, 400.0),
                rng.gen_range(2.0, 400.0),
                size,
                size,
                Vector::new(rng.gen_range(-40.0, 40.0), rng.gen_range(-40.0, 40.0)),
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
                let collision = a.collides_with(b);

                if let Some(vector) = collision {
                    let vector = vector.norm();
                    let a_collision_speed = vector.dot(&a.speed());
                    let b_collision_speed = vector.dot(&b.speed());

                    let a_kick = vector * (-2.0 * a_collision_speed);
                    let b_kick = vector * (-2.0 * b_collision_speed);

                    log::debug!("{:?}", vector);

                    log::debug!(
                        "({}, {}) -> {:?} ({:?}, {:?})",
                        i,
                        j,
                        collision,
                        a_collision_speed,
                        a_kick
                    );

                    self.objects[i].kick(a_kick);
                    self.objects[j].kick(b_kick);
                }
            }
        }
    }
}
