use super::{HitRecord, Hittable};
use crate::ray::{Interval, Ray};
use crate::vec3d::{Vec3d, Color};
use crate::object::aabb::AABB;
use crate::object::texture::Texture;
use crate::object::material;
use crate::object::material::Material;

use rand::{thread_rng, Rng};
use std::sync::Arc;


pub struct Medium {
    boundary: Arc<Box<dyn Hittable>>,
    neg_inv_density: f64,
    phase_func: Material,
}

impl Medium {
    pub fn new(boundary: Arc<Box<dyn Hittable>>, density: f64, phase_func: Arc<Box<dyn Texture>>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_func: Material::Isotropic(material::Isotropic::new(phase_func)),
        }
    }

    pub fn from_color(boundary: Arc<Box<dyn Hittable>>, density: f64, color: Vec3d) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_func: Material::Isotropic(material::Isotropic::from_color(color)),
        }
    }
}


impl Hittable for Medium {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let rec1 = self.boundary.hit(ray, &Interval::UNIVERSE);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1?;
        let rec2 = self.boundary.hit(ray, &Interval {min: rec1.t + 0.0001, max: f64::INFINITY});
        if rec2.is_none() {
            return None;
        }
        let mut rec2 = rec2?;

        rec1.t = rec1.t.max(interval.min);
        rec2.t = rec2.t.min(interval.max);

        if rec1.t >= rec2.t {
            return None;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let random_num = thread_rng().random::<f64>();
        let hit_distance = self.neg_inv_density * random_num.ln();

        if hit_distance < distance_inside_boundary {
            let t = rec1.t + hit_distance / ray_length;
            let record = HitRecord {
                t,
                u: 0.0,
                v: 0.0,
                point: ray.at(t),
                normal: Vec3d::new(1.0, 0.0, 0.0), // arbitrary
                front_face: true, // arbitrary
                material: &self.phase_func,
            };
            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
