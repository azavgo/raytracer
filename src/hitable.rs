use crate::{ray::Ray, vec3::Vec3}; 

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HitRecord {
    t: f64, 
    p: Vec3, 
    normal: Vec3, 
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
        Self {
            t: t, 
            p: p, 
            normal: normal,
        }
    }

    pub fn t(self) -> f64 {
        self.t
    }

    pub fn p(self) -> Vec3 {
        self.p
    }

    pub fn normal(self) -> Vec3 {
        self.normal
    }
}

pub trait Hitable {
    fn hit(self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;  
}



