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
    fuss: f64,
}

impl Metal {
    pub fn new(albedo: Vec3d, fuss: f64) -> Self {
        Self { albedo, fuss }
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
        let mut reflected = reflect(&ray_in.direction, &hit_record.normal);
        reflected += reflected.unit_vector() + Vec3d::random().unit_vector() * self.fuss;

        scattered.clone_from(&Ray::new(hit_record.point, reflected));
        attenuation.clone_from(&self.albedo);
        if dot(&scattered.direction, &hit_record.normal) > 0.0 {
            true
        } else {
            false
        }
    }
}

fn reflect(v_in: &Vec3d,  normal: &Vec3d) -> Vec3d {
    *v_in - *normal * dot(v_in, normal) * 2.0
}


fn refract(v_in: &Vec3d, normal: &Vec3d, etai_over_etat: f64) -> Vec3d {
    let cos_theta = dot(-v_in, normal).min(1.0);
    let r_out_perp = (*v_in + *normal * cos_theta) * etai_over_etat;
    let r_out_parallel = *normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}