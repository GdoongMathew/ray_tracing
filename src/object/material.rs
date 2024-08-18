use rand::random;
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
    Empty(Empty),
    Light(Light),
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            Material::Empty(e) => e.scatter(ray_in, hit_record, attenuation, scattered),
            Material::Light(li) => li.scatter(ray_in, hit_record, attenuation, scattered),
            Material::Lambertian(l) => l.scatter(ray_in, hit_record, attenuation, scattered),
            Material::Metal(metal) => metal.scatter(ray_in, hit_record, attenuation, scattered),
            Material::Dielectric(d) => d.scatter(ray_in, hit_record, attenuation, scattered),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Empty {}

impl Scatterable for Empty {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3d, scattered: &mut Ray) -> bool {
        false
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

        scattered.clone_from(&Ray::new(hit_record.point, scatter_direction, ray_in.time));
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
        if fuss > 1.0 {
            panic!("Fuss must be less than 1.0, get {} instead.", fuss);
        }
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
        reflected = reflected.unit_vector() + Vec3d::random().unit_vector() * self.fuss;

        scattered.clone_from(&Ray::new(hit_record.point, reflected, ray_in.time));
        attenuation.clone_from(&self.albedo);
        dot(&scattered.direction, &hit_record.normal) > 0.0
    }
}

fn reflect(v_in: &Vec3d, normal: &Vec3d) -> Vec3d {
    *v_in - *normal * dot(v_in, normal) * 2.0
}


#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}


impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}


impl Scatterable for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3d,
        scattered: &mut Ray,
    ) -> bool {
        let ri = if hit_record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = dot(&-unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, ri) > random() {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, ri)
        };
        attenuation.clone_from(&Vec3d::new(1.0, 1.0, 1.0));
        scattered.clone_from(&Ray::new(hit_record.point, direction, ray_in.time));
        true
    }
}


fn refract(v_in: &Vec3d, normal: &Vec3d, etai_over_etat: f64) -> Vec3d {
    let cos_theta = dot(&-*v_in, normal).min(1.0);
    let r_out_perp = (*v_in + *normal * cos_theta) * etai_over_etat;
    let r_out_parallel = *normal * -1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // use Schlick's approximation for reflectance
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}


#[cfg(test)]
mod test_scatter_fn {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_reflect_output_1() {
        let v_in = Vec3d::new(1.0, 1.0, 0.0);
        let normal = Vec3d::new(-1.0, 0.0, 0.0);
        let expected = Vec3d::new(-1.0, 1.0, 0.0);
        let result = reflect(&v_in, &normal);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reflect_output_2() {
        let v_in = Vec3d::new(1.0, 1.0, 0.0);
        let normal = Vec3d::new(0.0, 1.0, 0.0);
        let expected = Vec3d::new(1.0, -1.0, 0.0);
        let result = reflect(&v_in, &normal);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reflect_output_3() {
        let v_in = Vec3d::new(1.0, 1.0, 0.0);
        let normal = Vec3d::new(-1.0, -1.0, 0.0);
        let expected = Vec3d::new(-1.0, -1.0, 0.0);
        let result = reflect(&v_in, &normal.unit_vector());
        assert_approx_eq!(result.x(), expected.x(), f32::EPSILON as f64);
        assert_approx_eq!(result.y(), expected.y(), f32::EPSILON as f64);
        assert_eq!(result.z(), expected.z());
    }


    #[test]
    fn test_refract_perp_1() {
        let v_in = Vec3d::new(0.0, -1.0, 0.0);
        let normal = Vec3d::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.0;

        let expected = Vec3d::new(0.0, -1.0, 0.0); // No refraction, same vector
        let result = refract(&v_in, &normal, etai_over_etat);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_refract_perp_2() {
        let v_in = Vec3d::new(0.0, -1.0, 0.0);
        let normal = Vec3d::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.5;

        let expected = Vec3d::new(0.0, -1.0, 0.0); // No refraction, same vector
        let result = refract(&v_in, &normal, etai_over_etat);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_refract_perp_3() {
        let v_in = Vec3d::new(0.0, -1.0, 0.0);
        let normal = Vec3d::new(0.0, 1.0, 0.0);
        let etai_over_etat = 0.5;

        let expected = Vec3d::new(0.0, -1.0, 0.0); // No refraction, same vector
        let result = refract(&v_in, &normal, etai_over_etat);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_refract_1() {
        let v_in = Vec3d::new(1.0, 1.0, 0.0);
        let normal = Vec3d::new(-1.0, 0.0, 0.0);
        let etai_over_etat = 1.0;

        let expected = Vec3d::new(0.0, 1.0, 0.0); // Corrected expected result
        let result = refract(&v_in, &normal, etai_over_etat);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_refract_2() {
        let v_in = Vec3d::new(1.0, 1.0, 0.0);
        let normal = Vec3d::new(-1.0, 0.0, 0.0);
        let etai_over_etat = 0.5;

        let expected = Vec3d::new(0.8660254037844386, 0.5, 0.0); // Corrected expected result
        let result = refract(&v_in, &normal, etai_over_etat);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_refract_3() {
        let v_in = Vec3d::new(0.0, -1.0, 0.0);
        let normal = Vec3d::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.5;

        let expected = Vec3d::new(0.0, -1.0, 0.0); // Corrected expected result
        let result = refract(&v_in, &normal, etai_over_etat);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_material {
    use super::*;
    use crate::vec3d::Vec3d;

    #[test]
    fn test_empty_material() {
        let empty = Empty {};
        let ray_in = Ray::new(
            Vec3d::zero(),
            Vec3d::zero(),
            0.0,
        );
        let hit_record = HitRecord::empty();
        let mut attenuation = Vec3d::zero();
        let mut scattered = Ray::new(
            Vec3d::zero(),
            Vec3d::zero(),
            0.0,
        );

        let ret = empty.scatter(&ray_in, &hit_record, &mut attenuation, &mut scattered);
        assert_eq!(ret, false);
    }
}
