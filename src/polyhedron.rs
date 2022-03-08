use crate::{
    collisions::{collide_poly_circle, collide_poly_poly, collide_poly_rect},
    geometry::{Rectangle, Vector},
    object::MetaObject,
};

pub struct PolyhedronObject {
    pub center: Vector,
    pub points: Box<[Vector]>,
    pub speed: Vector,
    pub aabb: Rectangle,
}

impl PolyhedronObject {
    pub fn new(center: Vector, radius: f64, point_count: usize, speed: Vector) -> Self {
        let mut points = vec![Vector::zero(); point_count].into_boxed_slice();

        let mut angle = 0.0_f64;
        let angle_step = 2.0 * std::f64::consts::PI / point_count as f64;

        let mut aabb = Rectangle::new(f64::MAX, f64::MAX, f64::MIN, f64::MIN);

        for point in points.iter_mut() {
            let p = Vector::new(angle.sin(), angle.cos()) * radius;

            if p.x < aabb.coord.x {
                aabb.coord.x = p.x;
            }
            if p.x > aabb.size.x {
                aabb.size.x = p.x;
            }

            if p.y < aabb.coord.y {
                aabb.coord.y = p.y;
            }
            if p.y > aabb.size.y {
                aabb.size.y = p.y;
            }

            *point = p;
            angle += angle_step;
        }

        let aabb_size = aabb.size - aabb.coord;
        aabb.size = aabb_size;

        Self {
            center,
            points,
            speed,
            aabb,
        }
    }

    pub fn new_obj(center: Vector, radius: f64, point_count: usize, speed: Vector) -> MetaObject {
        MetaObject::Poly(Self::new(center, radius, point_count, speed))
    }

    pub fn tick(&mut self, delta_time: f64) {
        self.center += self.speed() * delta_time;
    }

    pub fn collides_with(&self, other: &MetaObject) -> Option<Vector> {
        match other {
            MetaObject::Rect(rect) => collide_poly_rect(self, rect),
            MetaObject::Circle(circle) => collide_poly_circle(self, circle),
            MetaObject::Poly(poly) => collide_poly_poly(self, poly),
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
        let mut aabb = self.aabb;
        aabb.coord += self.center;
        // log::info!("{:?}", aabb);
        aabb
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn points<'a>(&'a self) -> impl Iterator<Item = Vector> + 'a {
        self.points.iter().map(|it| *it + self.center)
    }

    pub fn points_len(&self) -> usize {
        self.points.len()
    }

    #[allow(clippy::needless_lifetimes)]
    pub fn lines<'a>(&'a self) -> impl Iterator<Item = Rectangle> + 'a {
        (0..self.points.len()).map(|i| {
            let j = (i + 1) % self.points.len();

            let p_len = (self.points[i] - self.points[j]).len();
            let p_vec = (self.points[i] - self.points[j]) * (1.0 / p_len);

            Rectangle::new_vec(self.points[i] + self.center, p_vec)
        })
    }
}
