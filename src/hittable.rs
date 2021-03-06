use crate::material::Material;
use crate::ray::*;
use crate::vec3::*;

#[derive(Default)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<&'a dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
