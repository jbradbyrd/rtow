use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    mat_ptr: Box<dyn Material + 'a>,
}

impl Sphere<'_> {
    pub fn new(cen: Point3, r: f64, m: Box<dyn Material>) -> Self {
        Sphere {
            center: cen,
            radius: r,
            mat_ptr: m,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Some(self.mat_ptr.as_ref());

        return Some(rec);
    }
}
