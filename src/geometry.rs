#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y()
    }

    pub fn dot_sqr(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2)
    }

    pub fn len(&self) -> f64 {
        self.dot_sqr().sqrt()
    }

    pub fn norm(&self) -> Self {
        *self * (1.0 / self.len())
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, right: f64) -> Self {
        Self::new(self.x * right, self.y * right)
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Self;
    fn add(self, right: Vector) -> Self {
        Self::new(self.x + right.x, self.y + right.y)
    }
}

impl std::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, right: Vector) {
        self.x += right.x;
        self.y += right.y;
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, right: Vector) -> Self {
        Self::new(self.x - right.x, self.y - right.y)
    }
}

impl std::ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, right: Vector) {
        self.x -= right.x;
        self.y -= right.y;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub coord: Vector,
    pub size: Vector,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            coord: Vector::new(x, y),
            size: Vector::new(w, h),
        }
    }

    pub fn new_vec(coord: Vector, size: Vector) -> Self {
        Self { coord, size }
    }

    pub fn left(&self) -> f64 {
        self.coord.x
    }

    pub fn right(&self) -> f64 {
        self.coord.x + self.size.x
    }

    pub fn top(&self) -> f64 {
        self.coord.y
    }

    pub fn bottom(&self) -> f64 {
        self.coord.y + self.size.y
    }

    pub fn center(&self) -> Vector {
        self.coord + self.coord * 0.5
    }

    pub fn points(&self) -> impl Iterator<Item = Vector> {
        [
            Vector::new(self.left(), self.top()),
            Vector::new(self.right(), self.top()),
            Vector::new(self.right(), self.bottom()),
            Vector::new(self.left(), self.bottom()),
        ]
        .into_iter()
    }

    pub fn collides_with(&self, other: &Rectangle) -> bool {
        !(self.left() > other.right()
            || self.right() < other.left()
            || self.top() > other.bottom()
            || self.bottom() < other.top())
    }
}

pub fn project_point(line: &Rectangle, point: Vector) -> f64 {
    (point - line.coord).dot(&line.size)
}

pub fn project_points<I: Iterator<Item = Vector>>(line: &Rectangle, points: I) -> Vector {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for point in points {
        let projection = project_point(line, point);
        if projection < min {
            min = projection;
        }
        if projection > max {
            max = projection;
        }
    }
    Vector::new(min, max)
}
