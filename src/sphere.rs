use crate::hitable::HitRecord; 
use crate::hitable::Hitable; 
use crate::vec3::Vec3;
use crate::ray::Ray;  

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    centre: Vec3, 
    radius: f64, 
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Self {
        Self {
            centre: centre, 
            radius: radius, 
        }
    }

    pub fn centre(self) -> Vec3 {
        self.centre
    }

    pub fn radius(self) -> f64 {
        self.radius
    }
}

impl Hitable for Sphere {
    fn hit(self, r: Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {
        let oc = r.origin() - self.centre(); 
        let a = Vec3::dot(r.direction(), r.direction()); 
        let half_b = Vec3::dot(oc, r.direction()); 
        let c = Vec3::dot(oc, oc) - self.radius() * self.radius(); 
        let quarter_d = (half_b * half_b - a * c); 
        
        if quarter_d < 0.0 {return false;}
        
        let sqrtd = quarter_d.sqrt(); 
        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min || t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min || t_max < root) {
                return false; 
            }
        }
        let t = root; 
        let p = r.point_at_parameter(root); 
        let normal = (p - self.centre()) / self.radius(); 
        rec = HitRecord::new(t, p, normal); 
        return true; 
    }
}
