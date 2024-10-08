use crate::ray::{Interval, Ray};
use super::hit::*;
use crate::vec3d::{Vec3d, Point3d, dot};
use crate::object::material::Material;
use crate::object::aabb::AABB;

pub struct Sphere {
    center: Point3d,
    radius: f64,
    material: Material,

    center_vec: Vec3d,
    bbox: AABB,
}

impl Sphere {
    pub fn static_sphere(
        center: Point3d,
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
        center: Point3d,
        center1: Point3d,
        radius: f64,
        material: Material,
    ) -> Self {
        let rvec = Vec3d::new(radius, radius, radius);
        let bbox = AABB::surrounding_box(
            &AABB::from_points(&(center - rvec), &(center + rvec)),
            &AABB::from_points(&(center1 - rvec), &(center1 + rvec)),
        );
        Self::new(center, center1, radius, material, bbox)
    }

    fn new(
        center: Point3d,
        center1: Point3d,
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

    pub fn sphere_center(&self, time: f64) -> Point3d {
        // If the sphere is not moving, the center is the same.
        self.center + self.center_vec * time
    }

    fn get_sphere_uv(point: &Vec3d) -> (f64, f64) {
        let theta = (-point.y()).acos();
        let phi = -point.z().atan2(point.x()) + std::f64::consts::PI;

        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
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
        if !interval.surrounds(root) {
            root = (h + sqrt_disc) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - center) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        let mut rec = HitRecord::new(&self.material, root, u, v, point);
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
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_sphere_outside_hit() {
        let sphere = Sphere::static_sphere(
            Point3d::new(0.0, 0.0, 0.0),
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

    // This function is directly copied from
    // https://github.com/fralken/ray-tracing-the-next-week/blob/ea3f3b5e2bb4e5967b7f6e1da415d5feffc4416a/src/sphere.rs#L9
    fn get_sphere_uv(p: &Vec3d) -> (f64, f64) {
        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        let v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;
        (u, v)
    }

    #[test]
    fn test_sphere_get_uv_1() {
        let point = Vec3d::new(0.0, 0.0, 1.0);

        let (u, v) = Sphere::get_sphere_uv(&point);
        let (target_u, target_v) = get_sphere_uv(&point);

        assert_eq!(u, target_u);
        assert_eq!(v, target_v);
    }

    #[test]
    fn test_sphere_get_uv_2() {
        let point = Vec3d::new(1.5, 2.0, 3.7).unit_vector();
        let (u, v) = Sphere::get_sphere_uv(&point);

        let (u, v) = Sphere::get_sphere_uv(&point);
        let (target_u, target_v) = get_sphere_uv(&point);

        assert_approx_eq!(u, target_u);
        assert_approx_eq!(v, target_v);
    }
}