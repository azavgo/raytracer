use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { a: a, b: b }
    }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}
