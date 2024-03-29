use rand::Rng;

use crate::{
    circle::CircleObject,
    geometry::{Rectangle, Vector},
    object::MetaObject,
    polyhedron::PolyhedronObject,
    quad_tree::QuadTree,
    rectangle::RectangleObject,
};

pub struct World {
    objects: Vec<MetaObject>,
    ticked: Vec<bool>,
    health: Vec<isize>,
    tick: bool,
    size: Vector,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let speed_rng = (30.0, 50.1);
        let size_rng = (1.0, 200.0);
        let rects = 1500;
        let circles = 1500;
        let polyhedrons = 1500;

        let mut objects = vec![
            // CircleObject::new_obj(Vector::new(200.0, 200.0), 50.0, Vector::new(-5.0, -5.0)),
            // PolyhedronObject::new_obj(Vector::new(60.0, 60.0), 50.0, 3, Vector::new(5.0, 5.0)),
            // CircleObject::new_obj(Vector::new(60.0, 60.0), 50.0, Vector::new(2.0, 2.0)),
            // RectangleObject::new_obj(100.0, 100.0, 30.0, 30.0, Vector::new(0.0, 0.0)),
            // RectangleObject::new_obj(10.0, 10.0, 30.0, 30.0, Vector::new(20.0, 20.0)),
            // PolyhedronObject::new_obj(Vector::new(20.0, 20.0), 30.0, 3, Vector::new(10.0, 10.0)),
            // CircleObject::new_obj(Vector::new(140.0, 140.0), 10.0, Vector::new(-10.0, -10.0)),
        ];
        {
            let mut rng = rand::thread_rng();
            for _ in 0..rects {
                let size = rng.gen_range(size_rng.0, size_rng.1);
                let speed = Vector::new(
                    rng.gen_range(speed_rng.0, speed_rng.1),
                    rng.gen_range(speed_rng.0, speed_rng.1),
                ) * rng.gen_range(-1.0_f64, 1.0).signum();

                objects.push(RectangleObject::new_obj(
                    rng.gen_range(0.0, width - size),
                    rng.gen_range(0.0, height - size),
                    size,
                    size,
                    speed,
                ))
            }
            for _ in 0..circles {
                let size = rng.gen_range(size_rng.0 / 2.0, size_rng.1 / 2.0);
                let speed = Vector::new(
                    rng.gen_range(speed_rng.0, speed_rng.1),
                    rng.gen_range(speed_rng.0, speed_rng.1),
                ) * rng.gen_range(-1.0_f64, 1.0).signum();

                objects.push(CircleObject::new_obj(
                    Vector::new(
                        rng.gen_range(size / 2.0, width - size / 2.0),
                        rng.gen_range(size / 2.0, height - size / 2.0),
                    ),
                    size,
                    speed,
                ))
            }
            for _ in 0..polyhedrons {
                let size = rng.gen_range(size_rng.0 / 2.0, size_rng.1 / 2.0);
                let speed = Vector::new(
                    rng.gen_range(speed_rng.0, speed_rng.1),
                    rng.gen_range(speed_rng.0, speed_rng.1),
                ) * rng.gen_range(-1.0_f64, 1.0).signum();

                objects.push(PolyhedronObject::new_obj(
                    Vector::new(
                        rng.gen_range(size / 2.0, width - size / 2.0),
                        rng.gen_range(size / 2.0, height - size / 2.0),
                    ),
                    size,
                    rng.gen_range(3, 10),
                    speed,
                ))
            }
        }
        Self {
            ticked: vec![false; objects.len()],
            health: vec![3; objects.len()],
            tick: true,
            objects,
            size: Vector::new(width, height),
        }
    }

    pub fn export_quad_tree(&self) -> Box<[f64]> {
        let mut quad_tree = QuadTree::new(Rectangle::new_vec(Vector::zero(), self.size));
        for (id, obj) in self.objects.iter().enumerate() {
            if self.health[id] <= 0 {
                continue;
            }
            quad_tree.insert(id, obj.aabb());
        }
        let mut vec = Vec::new();
        quad_tree.export(&mut vec);
        vec.into_boxed_slice()
    }

    pub fn export(&self) -> Box<[f64]> {
        let mut vec = Vec::with_capacity(self.objects.len() * 4);

        for (idx, object) in self.objects.iter().enumerate() {
            let health = self.health[idx];
            if health <= 0 {
                continue;
            }
            match object {
                MetaObject::Rect(rect) => {
                    vec.push(1.0);
                    vec.push(health as f64);
                    vec.push(rect.left());
                    vec.push(rect.top());
                    vec.push(rect.shape.size.x);
                    vec.push(rect.shape.size.y);
                }
                MetaObject::Circle(circle) => {
                    vec.push(2.0);
                    vec.push(health as f64);
                    vec.push(circle.center.x);
                    vec.push(circle.center.y);
                    vec.push(circle.radius);
                }
                MetaObject::Poly(poly) => {
                    vec.push(3.0);
                    vec.push(health as f64);
                    vec.push(poly.points_len() as f64);
                    for point in poly.points() {
                        vec.push(point.x);
                        vec.push(point.y);
                    }
                    let aabb = poly.aabb();
                    vec.push(aabb.left());
                    vec.push(aabb.top());
                    vec.push(aabb.size.x);
                    vec.push(aabb.size.y);
                }
            }
        }

        vec.into_boxed_slice()
    }

    pub fn tick(&mut self, delta_time: f64) {
        // return;
        let mut quad_tree = QuadTree::new(Rectangle::new_vec(Vector::zero(), self.size));
        for (id, obj) in self.objects.iter().enumerate() {
            if self.health[id] <= 0 {
                continue;
            }
            quad_tree.insert(id, obj.aabb());
        }

        let mut sum = 0.0;
        for i in 0..self.objects.len() {
            self.ticked[i] = self.tick;
            if self.health[i] <= 0 {
                continue;
            }

            'inner: for j in quad_tree.might_collide(i, self.objects[i].aabb()) {
            //'inner: for j in (i+1)..self.objects.len() {
                if self.ticked[j] == self.tick {
                    continue;
                }
                if self.health[j] <= 0  {
                    continue 'inner;
                }
                sum += 1.0;
                let a = &self.objects[i];
                let b = &self.objects[j];
                if !a.aabb().collides_with(&b.aabb()) {
                    continue 'inner;
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

                        self.health[i] -= 1;
                        self.health[j] -= 1;
                    }
                }
            }

            let aabb = self.objects[i].aabb();
            let speed = self.objects[i].speed();
            let mut kick = Vector::zero();
            let mut mov = Vector::zero();
            if aabb.left() < 0.0 {
                kick.x = speed.x * -2.0;
                mov.x = -aabb.left();
            }
            if aabb.right() > self.size.x {
                kick.x = speed.x * -2.0;
                mov.x = self.size.x - aabb.right();
            }
            if aabb.top() < 0.0 {
                kick.y = speed.y * -2.0;
                mov.y = -aabb.top();
            }
            if aabb.bottom() > self.size.y {
                kick.y = speed.y * -2.0;
                mov.y = self.size.y - aabb.bottom();
            }

            self.objects[i].kick(kick);
            self.objects[i].mov(mov);
        }

        log::debug!("{}", sum / self.objects.len() as f64);

        for (idx, obj) in self.objects.iter_mut().enumerate() {
            if self.health[idx] <= 0 {
                continue;
            }
            obj.tick(delta_time);
        }

        self.tick = !self.tick;
    }
}
