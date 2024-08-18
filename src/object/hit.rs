use crate::vec3d::{Vec3d, dot};
use crate::ray::{Ray, Interval};
use crate::object::aabb::AABB;
use super::material::{Material, Empty};



#[derive(Debug, Clone, Copy)]
pub struct HitRecord<'m> {
    pub t: f64,
    pub point: Vec3d,
    pub normal: Vec3d,
    pub front_face: bool,

    pub material: &'m Material,
}

impl<'m> HitRecord<'m> {
    pub fn new(material: &'m Material, t: f64, point: Vec3d) -> Self {
        Self {
            t,
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
            Vec3d::zero(),
        )
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3d) {
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}


pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}


pub struct HittableVec {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableVec {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::empty(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
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


#[derive(Copy, Clone)]
pub struct BVHNode<'node> {
    left: &'node Box<dyn Hittable>,
    right: &'node Box<dyn Hittable>,
    bbox: AABB,
}


impl BVHNode {
    pub fn from_hittable_vec(hittable_vec: &HittableVec) -> Self {
        Self::new(
            &hittable_vec.objects,
            0,
            hittable_vec.objects.len(),
        )
    }

    pub fn new(
        hittable_vec: &Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_index(0..3);

        let left: &Box<dyn Hittable>;
        let right: &Box<dyn Hittable>;

        let object_span = end - start;

        match object_span {
            1 => {
                left = &hittable_vec[start];
                right = &hittable_vec[start];
            }
            2 => {
                left = &hittable_vec[start];
                right = &hittable_vec[start + 1];
            }
            3 => {
                let mut new_hittable = &hittable_vec[start..end].to_vec();

                new_hittable.sort_by(|a, b| {
                    BVHNode::box_compare(a, b, axis)
                });

                let mid = start + object_span / 2;
                left = &Box::new(BVHNode::new(&new_hittable, start, mid));
                right = &Box::new(BVHNode::new(&new_hittable, mid, end));
            }
            _ => panic!("Invalid axis: {}", axis),
        }

        let bbox = AABB::surrounding_box(
            &left.bounding_box(),
            &right.bounding_box(),
        );
        Self { left, right, bbox }
    }

    fn box_compare(
        box_a: &Box<dyn Hittable>,
        box_b: &Box<dyn Hittable>,
        axis: usize,
    ) -> Ordering {
        let a_axis_interval = box_a.bounding_box().axis_interval(axis);
        let b_axis_interval = box_b.bounding_box().axis_interval(axis);
        a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if self.bbox.hit(ray, interval).is_none() {
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
