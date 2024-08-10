use crate::ray::{Interval, Ray};
use super::hit::*;
use crate::vec3d::{Vec3d, dot};

pub struct Sphere {
    center: Vec3d,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64) -> Self {
        if radius <= 0.0 {
            panic!("Radius must be greater than 0, but was {} instead.", radius);
        }
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;

        let a = ray.direction.length_squared();
        let h = dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - sqrt_disc) / a;
        if !(interval.surrounds(root)) {
            let root = (h + sqrt_disc) / a;
            if !(interval.surrounds(root)) {
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Some(rec)
    }
}