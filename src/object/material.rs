use crate::vec3d::{Vec3d, dot};
use crate::ray::Ray;
use crate::object::hit::HitRecord;

pub trait Scatterable {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3d,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Light(Light),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3d,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Light(li) => li.scatter(ray_in, hit_record, attenuation, scattered),
            Material::Lambertian(l) => l.scatter(ray_in, hit_record, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(ray_in, hit_record, attenuation, scattered),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    color: Vec3d,
}

impl Light {
    pub fn new() -> Self {
        Self {
            color: Vec3d::new(1.0, 1.0, 1.0),
        }
    }
}

impl Scatterable for Light {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3d,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3d,
}

impl Lambertian {
    pub fn new(albedo: Vec3d) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3d,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3d::random().unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction.clone_from(&hit_record.normal);
        }

        scattered.clone_from(&Ray::new(hit_record.point, scatter_direction));
        attenuation.clone_from(&self.albedo);
        true
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3d,
}

impl Metal {
    pub fn new(albedo: Vec3d) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3d,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&ray_in.direction, &hit_record.normal);

        scattered.clone_from(&Ray::new(hit_record.point, reflected));
        attenuation.clone_from(&self.albedo);
        true
    }
}

fn reflect(v_in: &Vec3d,  normal: &Vec3d) -> Vec3d {
    *v_in - *normal * dot(v_in, normal) * 2.0
}