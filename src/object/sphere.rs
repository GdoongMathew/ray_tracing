use crate::ray::{Interval, Ray};
use super::hit::*;
use crate::vec3d::{Vec3d, dot};
use crate::object::material::Material;
use crate::object::aabb::AABB;

pub struct Sphere {
    center: Vec3d,
    radius: f64,
    material: Material,

    center_vec: Vec3d,
    bbox: AABB,
}

impl Sphere {
    pub fn static_sphere(
        center: Vec3d,
        radius: f64,
        material: Material,
    ) -> Self {
        let bbox = AABB::from_points(
            &(center - Vec3d::new(radius, radius, radius)),
            &(center + Vec3d::new(radius, radius, radius)),
        );
        Self::new(center, center, radius, material, bbox)
    }

    pub fn moving_sphere(
        center: Vec3d,
        center1: Vec3d,
        radius: f64,
        material: Material,
    ) -> Self {

        let rvec = Vec3d::new(radius, radius, radius);
        let bbox = AABB::surrounding_box(
            &AABB::from_points(&(center - rvec), &(center + rvec)),
            &AABB::from_points(&(center1 - rvec), &(center1 + rvec))
        );
        Self::new(center, center1, radius, material, bbox)
    }

    fn new(
        center: Vec3d,
        center1: Vec3d,
        radius: f64,
        material: Material,
        bbox: AABB,
    ) -> Self {
        if radius <= 0.0 {
            panic!("Radius must be greater than 0, but was {} instead.", radius);
        }
        Self {
            center,
            radius,
            material,
            center_vec: center1 - center,
            bbox,
        }
    }

    pub fn is_moving(&self) -> bool {
        self.center_vec.x() != 0.0 || self.center_vec.y() != 0.0 || self.center_vec.z() != 0.0
    }

    pub fn sphere_center(&self, time: f64) -> Vec3d {
        // If the sphere is not moving, the center is the same.
        self.center + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {

        let center = if self.is_moving() {
            self.sphere_center(ray.time)
        } else {
            self.center
        };
        let oc = center - ray.origin;

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

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}


#[cfg(test)]
mod test_hittable {
    use super::*;

    use super::super::material::*;

    #[test]
    fn test_sphere_outside_hit() {
        let sphere = Sphere::static_sphere(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(0.0, 0.0, -5.0),
            Vec3d::new(0.0, 0.0, 1.0),
            0.0,
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
            let sphere = Sphere::static_sphere(
                Vec3d::new(0.0, 0.0, 0.0),
                2.0,
                Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
            );
            let ray = Ray::new(
                Vec3d::new(0.0, 0.0, 0.0),
                Vec3d::new(0.0, 0.0, 1.0),
                0.0,
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
        let sphere = Sphere::static_sphere(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(0.0, 0.0, -5.0),
            Vec3d::new(0.0, 0.0, -1.0),
            0.0,
        );
        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = sphere.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }

    #[test]
    fn test_sphere_no_hit_2() {
        let sphere = Sphere::static_sphere(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );
        let ray = Ray::new(
            Vec3d::new(2.0, 2.0, -1.0),
            Vec3d::new(2.0, 0.0, -1.0),
            0.0,
        );
        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = sphere.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }
}