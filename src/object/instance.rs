use crate::vec3d::{Vec3d, Point3d};
use super::{HitRecord, Hittable};
use crate::object::aabb::AABB;
use crate::ray::{Interval, Ray};

use std::sync::Arc;

pub struct Translate {
    offset: Vec3d,
    object: Arc<Box<dyn Hittable>>,
    bbox: AABB,
}


impl Translate {
    pub fn new(object: Arc<Box<dyn Hittable>>, offset: Vec3d) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            offset,
            object,
            bbox,
        }
    }
}


impl Hittable for Translate {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(
            ray.origin - self.offset,
            ray.direction,
            ray.time,
        );

        if let Some(mut hit_record) = self.object.hit(&offset_ray, interval) {
            hit_record.point += self.offset;
            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}


#[cfg(test)]
mod test_translate {
    use super::*;
    use crate::vec3d::Point3d;
    use crate::object::Quad;
    use crate::object::material;
    use crate::object::material::Material;

    #[test]
    fn test_translate_hit() {
        let quad = Quad::new(
            Point3d::zero(),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Empty(material::Empty {}),
        );
        let translate = Translate::new(Arc::new(
            Box::new(quad)), Vec3d::new(1.0, 0.0, 0.0),
        );

        assert_eq!(
            translate.bounding_box(),
            AABB::from_points(
                &Point3d::new(1.0, 0.0, 0.0),
                &Point3d::new(2.0, 1.0, 0.0),
            )
        )
    }
}


pub struct RotateY {
    object: Arc<Box<dyn Hittable>>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}


impl RotateY {
    pub fn new(object: Arc<Box<dyn Hittable>>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();
        let mut min = Point3d::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3d::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.axis_interval(0).max +
                        (1 - i) as f64 * bbox.axis_interval(0).min;
                    let y = j as f64 * bbox.axis_interval(1).max +
                        (1 - j) as f64 * bbox.axis_interval(1).min;
                    let z = k as f64 * bbox.axis_interval(2).max +
                        (1 - k) as f64 * bbox.axis_interval(2).min;
                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Point3d::new(new_x, y, new_z);

                    for c in 0..3{
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::from_points(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let origin = Point3d::new(
            self.cos_theta * ray.origin.x() - self.sin_theta * ray.origin.z(),
            ray.origin.y(),
            self.sin_theta * ray.origin.x() + self.cos_theta * ray.origin.z(),
        );

        let direction = Vec3d::new(
            self.cos_theta * ray.direction.x() - self.sin_theta * ray.direction.z(),
            ray.direction.y(),
            self.sin_theta * ray.direction.x() + self.cos_theta * ray.direction.z(),
        );

        let rotated_ray = Ray::new(
            origin, direction, ray.time,
        );

        if let Some(mut hit_record) = self.object.hit(&rotated_ray, interval) {
            hit_record.point = Point3d::new(
                self.cos_theta * hit_record.point.x() + self.sin_theta * hit_record.point.z(),
                hit_record.point.y(),
                -self.sin_theta * hit_record.point.x() + self.cos_theta * hit_record.point.z(),
            );

            hit_record.normal = Vec3d::new(
                self.cos_theta * hit_record.normal.x() + self.sin_theta * hit_record.normal.z(),
                hit_record.normal.y(),
                (-self.sin_theta * hit_record.normal.x() + self.cos_theta * hit_record.normal.z()),
            );

            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

