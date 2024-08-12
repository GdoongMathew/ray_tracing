use crate::vec3d::{Vec3d, dot};
use crate::ray::{Ray, Interval};
use super::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord<'m> {
    pub t: f64,
    pub point: Vec3d,
    pub normal: Vec3d,
    pub front_face: bool,

    pub material: &'m Material,
}

impl <'m> HitRecord<'m> {

    pub fn new(material: &'m Material, t: f64, point: Vec3d) -> Self {
        Self {
            t,
            point,
            normal: Vec3d::zero(),
            front_face: false,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3d) {
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
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
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = interval.max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, &Interval{min: interval.min, max: closest_so_far}) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }
}
