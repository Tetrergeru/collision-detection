#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn right(&self) -> f64 {
        self.x + self.w
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.h
    }
}

impl Vector {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y()
    }

    pub fn len(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
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

pub fn one_dimensional_collision(a: Vector, b: Vector) -> f64 {
    if a.y < b.x || a.x > b.y {
        return f64::NAN;
    }
    if a.y < b.y && a.x < b.x {
        return b.x - a.y;
    }
    if a.y > b.y && a.x > b.x {
        return b.y - a.x;
    }
    0.0
}
