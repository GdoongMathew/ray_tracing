use crate::vec3d::{Vec3d, dot};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3d,
    pub normal: Vec3d,
    front_face: bool,
}

impl HitRecord {

    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Vec3d::zero(),
            normal: Vec3d::zero(),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3d) {
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}


pub struct HittableVec {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableVec {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableVec {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, t_max, &mut hit_record) {
                hit_anything = true;
                closest_so_far = hit_record.t;
                rec.clone_from(&hit_record);
            }
        }

        hit_anything
    }
}
