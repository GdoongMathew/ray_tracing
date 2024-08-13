use crate::ray::{Interval, Ray};
use super::hit::*;
use crate::vec3d::{Vec3d, dot};
use crate::object::material::Material;

pub struct Sphere {
    center: Vec3d,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64, material: Material) -> Self {
        if radius <= 0.0 {
            panic!("Radius must be greater than 0, but was {} instead.", radius);
        }
        Self { center, radius, material }
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
        let mut root = (h - sqrt_disc) / a;
        if !(interval.surrounds(root)) {
            root = (h + sqrt_disc) / a;
            if !(interval.surrounds(root)) {
                return None;
            }
        }

        let mut rec = HitRecord::new(&self.material, root, ray.at(root));
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Some(rec)
    }
}


#[cfg(test)]
mod test_hittable {
    use super::*;

    use super::super::material::*;

    #[test]
    fn test_sphere_outside_hit() {
        let sphere = Sphere::new(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(0.0, 0.0, -5.0),
            Vec3d::new(0.0, 0.0, 1.0),
        );
        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = sphere.hit(&ray, &interval).unwrap();

        assert_eq!(hit_record.t, 3.0);
        assert_eq!(hit_record.point, Vec3d::new(0.0, 0.0, -2.0));
        assert_eq!(hit_record.normal, Vec3d::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.front_face, true);
    }

    #[test]
    fn test_sphere_inside_hit() {
        {
        let sphere = Sphere::new(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(0.0, 0.0, 0.0),
            Vec3d::new(0.0, 0.0, 1.0),
        );
        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = sphere.hit(&ray, &interval).unwrap();

        assert_eq!(hit_record.t, 2.0);
        assert_eq!(hit_record.point, Vec3d::new(0.0, 0.0, 2.0));
        assert_eq!(hit_record.normal, Vec3d::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.front_face, false);
    }
    }

    #[test]
    fn test_sphere_no_hit_1() {
        let sphere = Sphere::new(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(0.0, 0.0, -5.0),
            Vec3d::new(0.0, 0.0, -1.0),
        );
        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = sphere.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }

    #[test]
    fn test_sphere_no_hit_2() {
        let sphere = Sphere::new(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(2.0, 2.0, -1.0),
            Vec3d::new(2.0, 0.0, -1.0),
        );
        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = sphere.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }
}