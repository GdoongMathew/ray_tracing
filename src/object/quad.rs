use crate::vec3d::{Vec3d, Point3d, cross, dot};

use crate::object::aabb::AABB;
use crate::object::HitRecord;
use crate::object::material::Material;
use crate::ray::{Interval, Ray};
use crate::object::hit::Hittable;


pub struct Quad {
    point: Point3d,
    vec_u: Vec3d,
    vec_v: Vec3d,
    vec_w: Vec3d,

    normal: Vec3d,
    shift_d: f64,

    material: Material,
    bbox: AABB,
}

impl Quad {
    pub fn new(point: Point3d, vec_u: Vec3d, vec_v: Vec3d, material: Material) -> Self {
        let n = cross(&vec_u, &vec_v);
        let normal = n.unit_vector();
        let shift_d = dot(&normal, &point);
        let vec_w = n / dot(&n, &n);

        let bbox = Self::get_bounding_box(&point, &vec_u, &vec_v);

        Self {
            point,
            vec_u,
            vec_v,
            vec_w,
            normal,
            shift_d,
            material,
            bbox,
        }
    }

    fn get_bounding_box(point: &Point3d, vec_u: &Vec3d, vec_v: &Vec3d) -> AABB {
        let bbox_diagonal_1 = AABB::from_points(
            point, &(*point + *vec_u + *vec_v),
        );
        let bbox_diagonal_2 = AABB::from_points(
            &(*point + *vec_u), &(*point + *vec_v),
        );
        AABB::surrounding_box(&bbox_diagonal_1, &bbox_diagonal_2)
    }

    fn is_interior(alpha: f64, beta: f64) -> bool {
        let unit_interval = Interval { min: 0.0, max: 1.0 };
        unit_interval.contains(alpha) && unit_interval.contains(beta)
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let denom = dot(&self.normal, &ray.direction);

        // Return None if ray is parallel to the plane, or the hit point parameter t
        // is outside the ray.
        if denom.abs() < f64::EPSILON { return None; };

        let t = (self.shift_d - dot(&self.normal, &ray.origin)) / denom;
        if !interval.contains(t) { return None; };

        let intersection = ray.at(t);

        // Determine if the hit point lies within the plane.
        let planar_hit_point_vector = intersection - self.point;
        let alpha = dot(&self.vec_w, &cross(&planar_hit_point_vector, &self.vec_v));
        let beta = dot(&self.vec_w, &cross(&self.vec_u, &planar_hit_point_vector));
        if !Self::is_interior(alpha, beta) { return None; };

        let mut rec = HitRecord::new(
            &self.material,
            t,
            alpha,
            beta,
            intersection,
        );
        rec.set_face_normal(ray, self.normal.clone());
        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}


#[cfg(test)]
mod test_quad {
    use super::*;
    use crate::object::material::Lambertian;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_quad_is_interval() {
        assert_eq!(Quad::is_interior(0.5, 0.5), true);
        assert_eq!(Quad::is_interior(0.0, 0.0), true);
        assert_eq!(Quad::is_interior(1.0, 1.0), true);
    }

    #[test]
    fn test_quad_not_is_interval() {
        assert_eq!(Quad::is_interior(1.1, 0.5), false);
        assert_eq!(Quad::is_interior(0.5, 1.1), false);
        assert_eq!(Quad::is_interior(-0.1, 0.5), false);
    }

    #[test]
    fn test_quad_get_bounding_box_1() {
        let point = Point3d::zero();
        let vec_u = Vec3d::new(1.0, 0.0, 0.0);
        let vec_v = Vec3d::new(0.0, 1.0, 0.0);

        let bbox = Quad::get_bounding_box(&point, &vec_u, &vec_v);
        let target = AABB::from_points(&point, &Vec3d::new(1.0, 1.0, 0.0));

        assert_eq!(bbox, target);
    }

    #[test]
    fn test_quad_get_bounding_box_2() {
        let point = Vec3d::new(10.0, 5.0, 2.0);
        let vec_u = Vec3d::new(2.0, 3.0, 0.0);
        let vec_v = Vec3d::new(0.0, 2.0, -2.0);

        let bbox = Quad::get_bounding_box(&point, &vec_u, &vec_v);
        let target = AABB::from_points(&point, &Vec3d::new(12.0, 10.0, 0.0));

        assert_eq!(bbox, target);
    }

    #[test]
    fn test_quad_init_1() {
        let quad = Quad::new(
            Vec3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        assert_eq!(quad.point, Vec3d::new(0.0, 0.0, 0.0));
        assert_eq!(quad.vec_u, Vec3d::new(1.0, 0.0, 0.0));
        assert_eq!(quad.vec_v, Vec3d::new(0.0, 1.0, 0.0));
        assert_eq!(quad.vec_w, Vec3d::new(0.0, 0.0, 1.0));
        assert_eq!(quad.normal, Vec3d::new(0.0, 0.0, 1.0));
        assert_eq!(quad.shift_d, 0.0);
    }

    #[test]
    fn test_quad_hit_1() {
        let quad = Quad::new(
            Point3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        let ray = Ray::new(
            Point3d::new(0.0, 0.0, -5.0),
            Vec3d::new(0.0, 0.0, 1.0),
            0.0,
        );

        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = quad.hit(&ray, &interval).unwrap();

        assert_eq!(hit_record.t, 5.0);
        assert_eq!(hit_record.point, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3d::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.front_face, false);
    }

    #[test]
    fn test_quad_hit_2() {
        let quad = Quad::new(
            Point3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        let ray = Ray::new(
            Point3d::new(0.0, 0.5, -7.5),
            Vec3d::new(0.0, 0.0, 1.0),
            0.0,
        );

        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = quad.hit(&ray, &interval).unwrap();

        assert_eq!(hit_record.t, 7.5);
        assert_eq!(hit_record.point, Point3d::new(0.0, 0.5, 0.0));
        assert_eq!(hit_record.normal, Vec3d::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.front_face, false);
    }

    #[test]
    fn test_quad_hit_3() {
        let quad = Quad::new(
            Point3d::new(-0.5, -0.5, 0.0),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        let ray = Ray::new(
            Point3d::new(-0.5, -0.5, -0.5),
            Vec3d::new(1.0, 1.0, 1.0).unit_vector(),
            0.0,
        );

        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = quad.hit(&ray, &interval).unwrap();

        assert_approx_eq!(hit_record.t, (0.5_f64.powi(2) * 3.0).sqrt());
        assert_eq!(hit_record.point, Point3d::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3d::new(0.0, 0.0, -1.0));
        assert_eq!(hit_record.front_face, false);
    }

    #[test]
    fn test_quad_not_hit_not_contain() {
        let quad = Quad::new(
            Point3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        let ray = Ray::new(
            Point3d::new(0.0, 0.0, -5.0),
            Vec3d::new(0.0, 0.0, 1.0),
            0.0,
        );

        let interval = Interval { min: 0.0, max: 4.0 };
        let hit_record = quad.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }

    #[test]
    fn test_quad_not_hit_parallel() {
        let quad = Quad::new(
            Point3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        let ray = Ray::new(
            Point3d::new(0.0, 0.5, 0.5),
            Vec3d::new(0.0, 0.0, 1.0),
            0.0,
        );

        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = quad.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }

    #[test]
    fn test_quad_not_hit_not_interior() {
        let quad = Quad::new(
            Point3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5))),
        );

        let ray = Ray::new(
            Point3d::new(0.0, 1.1, -5.0),
            Vec3d::new(0.0, 0.0, 1.0),
            0.0,
        );

        let interval = Interval { min: 0.0, max: f64::INFINITY };
        let hit_record = quad.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }
}