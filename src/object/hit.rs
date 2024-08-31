use crate::vec3d::{Point3d, Vec3d, dot};
use crate::ray::{Ray, Interval};
use crate::object::aabb::AABB;
use super::material::{Material, Empty};

use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

use std::default::Default;


#[derive(Debug, Clone, Copy)]
pub struct HitRecord<'m> {
    pub t: f64,
    pub u: f64,
    pub v: f64,

    pub point: Point3d,
    pub normal: Vec3d,
    pub front_face: bool,

    pub material: &'m Material,
}

impl<'m> HitRecord<'m> {
    pub fn new(material: &'m Material, t: f64, u: f64, v: f64, point: Point3d) -> Self {
        Self {
            t,
            u,
            v,
            point,
            normal: Vec3d::zero(),
            front_face: false,
            material,
        }
    }

    /// An empty hitrecord, simply for the purpose of determining whether
    /// an object is being hit without further hittable calculation.
    pub fn empty() -> Self {
        Self::new(
            &Material::Empty(Empty {}),
            0.0,
            0.0,
            0.0,
            Point3d::zero(),
        )
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3d) {
        // Determine if the ray is hitting the front or back face of the object
        // by checking the angle between the ray direction and the outward normal.
        // The `front_face` is true if the angle is less than 90 degrees.
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;

        // The normal vector is always facing the ray, so if the ray is hitting
        // the back face, the normal vector should be inverted.
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}


impl PartialEq for HitRecord<'_> {
    fn eq(&self, other: &Self) -> bool {
         self.t == other.t &&
            self.u == other.u &&
            self.v == other.v &&
            self.point == other.point &&
            self.normal == other.normal &&
            self.front_face == other.front_face &&
            self.material == other.material
    }
}


pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}


pub struct HittableVec {
    pub objects: Vec<Arc<Box<dyn Hittable>>>,
    bbox: AABB,
}

impl HittableVec {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::EMPTY,
        }
    }

    pub fn add(&mut self, object: Arc<Box<dyn Hittable>>) {
        self.bbox = AABB::surrounding_box(&self.bbox, &object.bounding_box());
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
            if let Some(rec) = object.hit(ray, &Interval { min: interval.min, max: closest_so_far }) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        hit_record
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}


pub struct BVHNode {
    left: Arc<Box<dyn Hittable>>,
    right: Arc<Box<dyn Hittable>>,
    bbox: AABB,
}


impl BVHNode {
    pub fn from_hittable_vec(hittable_vec: Arc<HittableVec>) -> Self {
        Self::new(
            hittable_vec.objects.clone(),
            0,
            hittable_vec.objects.len(),
        )
    }

    pub fn new(
        mut hittable_vec: Vec<Arc<Box<dyn Hittable>>>,
        start: usize,
        end: usize,
    ) -> Self {

        // Sort the hittable objects along the longest axis of the bounding box
        let mut bbox = AABB::EMPTY;
        for i in start..end {
            bbox = AABB::surrounding_box(&bbox, &hittable_vec[i].bounding_box());
        }
        let axis = bbox.longest_axis();

        let mut left: Arc<Box<dyn Hittable>>;
        let mut right: Arc<Box<dyn Hittable>>;

        let object_span = end - start;

        match object_span {
            1 => {
                left = hittable_vec[start].clone();
                right = hittable_vec[start].clone();
            }
            2 => {
                left = hittable_vec[start].clone();
                right = hittable_vec[start + 1].clone();
            }
            _ => {
                let mut hit_vec = hittable_vec.clone();
                hit_vec.sort_by(|a, b| {
                    BVHNode::box_compare(a, b, axis)
                });

                let mid = start + object_span / 2;

                let right_hittable = hittable_vec.drain(mid..end).collect();
                let left_hittable = hittable_vec.drain(start..mid).collect();

                left = Arc::new(Box::new(BVHNode::new(left_hittable, start - start, mid - start)));
                right = Arc::new(Box::new(BVHNode::new(right_hittable, mid - mid, end - mid)));
            }
        }

        Self { left, right, bbox }
    }

    fn box_compare(
        box_a: &Arc<Box<dyn Hittable>>,
        box_b: &Arc<Box<dyn Hittable>>,
        axis: usize,
    ) -> Ordering {
        let a_axis_interval = box_a.bounding_box().axis_interval(axis);
        let b_axis_interval = box_b.bounding_box().axis_interval(axis);
        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval) {
            return None;
        }

        let hit_left = self.left.hit(ray, interval);

        let mut right_interval = Interval {
            min: interval.min,
            max: if hit_left.is_some() { hit_left?.t } else { interval.max },
        };
        let hit_right = self.right.hit(ray, &mut right_interval);

        // Return the closest hit if both left and right hits are Some
        if hit_left.is_some() && hit_right.is_some() {
            if hit_left?.t < hit_right?.t {
                hit_left
            } else {
                hit_right
            }
        } else if hit_left.is_some() {
            hit_left
        } else {
            hit_right
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}


#[cfg(test)]
mod bvh_node_test {
    use super::*;
    use crate::ray::Interval;
    use crate::object::{Sphere, Quad};

    #[test]
    fn test_bvh_node_one_sphere_bounding_box() {
        let sphere = Sphere::static_sphere(
            Vec3d::new(0.0, 0.0, 0.0),
            1.0,
            Material::Empty(Empty {}),
        );

        let sphere_box = sphere.bounding_box();

        let object_vec: Vec<Arc<Box<dyn Hittable>>> = vec![
            Arc::new(Box::new(sphere)),
        ];
        let node = BVHNode::new(
            object_vec.clone(),
            0,
            object_vec.len(),
        );

        let bbox = node.bounding_box();

        let expect_box = AABB::new(
            Interval { min: -1.0, max: 1.0 },
            Interval { min: -1.0, max: 1.0 },
            Interval { min: -1.0, max: 1.0 }
        );

        assert_eq!(bbox, expect_box);
        assert_eq!(bbox, sphere_box);
    }

    #[test]
    fn test_bvh_node_two_sphere_bounding_box() {
        let object_vec: Vec<Arc<Box<dyn Hittable>>> = vec![
            Arc::new(Box::new(Sphere::static_sphere(
                Vec3d::new(-1.0, 0.0, -1.0),
                1.0,
                Material::Empty(Empty {}),
            ))),
            Arc::new(Box::new(Sphere::static_sphere(
                Vec3d::new(1.0, 1.0, 0.0),
                1.0,
                Material::Empty(Empty {}),
            ))),
        ];
        let node = BVHNode::new(
            object_vec.clone(),
            0,
            object_vec.len(),
        );

        let bbox = node.bounding_box();
        let expect_box = AABB::new(
            Interval { min: -2.0, max: 2.0 },
            Interval { min: -1.0, max: 2.0 },
            Interval { min: -2.0, max: 1.0 }
        );

        assert_eq!(bbox, expect_box);
    }

    #[test]
    fn test_bvh_node_one_quad_bounding_box() {
        let quad = Quad::new(
            Vec3d::new(0.0, 0.0, 0.0),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Material::Empty(Empty {}),
        );

        let quad_box = quad.bounding_box();

        let object_vec: Vec<Arc<Box<dyn Hittable>>> = vec![
            Arc::new(Box::new(quad)),
        ];

        let node = BVHNode::new(
            object_vec.clone(),
            0,
            object_vec.len(),
        );

        let bbox = node.bounding_box();
        let expect_box = AABB::new(
            Interval { min: 0.0, max: 1.0 },
            Interval { min: 0.0, max: 1.0 },
            Interval { min: 0.0, max: 0.0 }
        );
        assert_eq!(bbox, expect_box);
        assert_eq!(bbox, quad_box);
    }

    #[test]
    fn test_bvh_node_box_compare() {
        let a: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(-1.0, 0.0, -1.0),
            1.0,
            Material::Empty(Empty {}),
        )));
        let b: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(1.0, -1.0, 0.0),
            2.0,
            Material::Empty(Empty {}),
        )));

        let result_0 = BVHNode::box_compare(&a, &b, 0);
        let result_1 = BVHNode::box_compare(&a, &b, 1);
        let result_2 = BVHNode::box_compare(&a, &b, 2);
        assert_eq!(result_0, Ordering::Less);
        assert_eq!(result_1, Ordering::Greater);
        assert_eq!(result_2, Ordering::Equal);
    }

    #[test]
    fn test_bvh_node_box_sort() {
        let object_vec: Vec<Arc<Box<dyn Hittable>>> = vec![
            Arc::new(Box::new(Sphere::static_sphere(
                Vec3d::new(-1.0, 0.0, -1.0),
                1.0,
                Material::Empty(Empty {}),
            ))),
            Arc::new(Box::new(Sphere::static_sphere(
                Vec3d::new(1.0, -1.0, 0.0),
                2.0,
                Material::Empty(Empty {}),
            ))),
        ];

        let mut object_vec_clone = object_vec.clone();
        object_vec_clone.sort_by(|a, b| {
            BVHNode::box_compare(a, b, 0)
        });

        assert!(Arc::ptr_eq(&object_vec[0], &object_vec_clone[0]));
        assert!(Arc::ptr_eq(&object_vec[1], &object_vec_clone[1]));

        let mut object_vec_clone2 = object_vec.clone();
        object_vec_clone2.sort_by(|a, b| {
            BVHNode::box_compare(a, b, 1)
        });

        assert!(Arc::ptr_eq(&object_vec[0], &object_vec_clone2[1]));
        assert!(Arc::ptr_eq(&object_vec[1], &object_vec_clone2[0]));
    }
}
