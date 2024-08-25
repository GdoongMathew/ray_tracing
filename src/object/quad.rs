use crate::vec3d::{Vec3d, cross, dot};

use crate::object::aabb::AABB;
use crate::object::HitRecord;
use crate::object::material::Material;
use crate::ray::{Interval, Ray};
use crate::object::hit::Hittable;


pub struct Quad {
    point: Vec3d,
    vec_u: Vec3d,
    vec_v: Vec3d,
    vec_w: Vec3d,

    normal: Vec3d,
    shift_d: f64,

    material: Material,
    bbox: AABB,
}

impl Quad {
    pub fn new(point: Vec3d, vec_u: Vec3d, vec_v: Vec3d, material: Material) -> Self {
        let n = cross(&vec_u, &vec_v);
        let normal = n.unit_vector();
        let shift_d = dot(&normal, &point);
        let vec_w = n / dot(&n, &n);

        let bbox = Self::bounding_box(&point, &vec_u, &vec_v);

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

    fn bounding_box(point: &Vec3d, vec_u: &Vec3d, vec_v: &Vec3d) -> AABB {
        let bbox_diagonal_1 = AABB::from_points(
            point, &(*point + *vec_u + *vec_v),
        );
        let bbox_diagonal_2 = AABB::from_points(
            &(*point + *vec_u), &(*point + *vec_v),
        );
        AABB::surrounding_box(&bbox_diagonal_1, &bbox_diagonal_2)
    }

    fn is_interior(alpha: f64, beta: f64) -> bool {
        let unit_interval = Interval{min: 0.0, max: 1.0};
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
        if !Self::is_interior(alpha, beta) {return None;};

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
        todo!()
    }
}