use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn e() -> Self {
        Self {
            e: [0f64, 0f64, 0f64],
        }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn r(self) -> f64 {
        self.e[0]
    }

    pub fn g(self) -> f64 {
        self.e[1]
    }

    pub fn b(self) -> f64 {
        self.e[2]
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    pub fn squared_length(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn make_unit_vector(self) -> Self {
        let k = 1.0 / self.length();
        Self::new(k * self.e[0], k * self.e[1], k * self.e[2])
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

/************************/
// Vec3 (+-*/) f64 = Vec3
impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self::new(self.e[0] + other, self.e[1] + other, self.e[2] + other)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self::new(self.e[0] - other, self.e[1] - other, self.e[2] - other)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

/************************/
// f64 (+-*) Vec3 = Vec3
impl Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self + other.e[0], self + other.e[1], self + other.e[2])
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self - other.e[0], self - other.e[1], self - other.e[2])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

/************************/
// Vec3 (+-*/) Vec3 = Vec3
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}
