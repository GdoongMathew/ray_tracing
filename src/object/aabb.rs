use crate::ray::{Interval, Ray};
use crate::vec3d::Vec3d;
use std::ops::{Add, Sub};


/// Axis-aligned bounding box.
/// # Fields
/// * `interval_x` - The interval of x values.
/// * `interval_y` - The interval of y values.
/// * `interval_z` - The interval of z values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    interval_x: Interval,
    interval_y: Interval,
    interval_z: Interval,
}

impl Add<Vec3d> for AABB {
    type Output = Self;

    fn add(self, rhs: Vec3d) -> Self::Output {
        Self {
            interval_x: self.interval_x + rhs.x(),
            interval_y: self.interval_y + rhs.y(),
            interval_z: self.interval_z + rhs.z(),
        }
    }
}

impl Add<&Vec3d> for AABB {
    type Output = Self;

    fn add(self, rhs: &Vec3d) -> Self::Output {
        Self {
            interval_x: self.interval_x + rhs.x(),
            interval_y: self.interval_y + rhs.y(),
            interval_z: self.interval_z + rhs.z(),
        }
    }
}


impl Add<AABB> for AABB {
    type Output = Self;

    fn add(self, rhs: AABB) -> Self::Output {
        Self {
            interval_x: self.interval_x + rhs.interval_x,
            interval_y: self.interval_y + rhs.interval_y,
            interval_z: self.interval_z + rhs.interval_z,
        }
    }
}


impl Sub<Vec3d> for AABB {
    type Output = Self;

    fn sub(self, rhs: Vec3d) -> Self::Output {
        Self {
            interval_x: self.interval_x - rhs.x(),
            interval_y: self.interval_y - rhs.y(),
            interval_z: self.interval_z - rhs.z(),
        }
    }
}

impl Sub<&Vec3d> for AABB {
    type Output = Self;

    fn sub(self, rhs: &Vec3d) -> Self::Output {
        Self {
            interval_x: self.interval_x - rhs.x(),
            interval_y: self.interval_y - rhs.y(),
            interval_z: self.interval_z - rhs.z(),
        }
    }
}


impl AABB {
    /// Creates a new AABB.
    /// # Arguments
    /// * `interval_x` - The interval of x values.
    /// * `interval_y` - The interval of y values.
    /// * `interval_z` - The interval of z values.
    /// # Returns
    /// The new AABB.
    pub fn new(interval_x: Interval, interval_y: Interval, interval_z: Interval) -> Self {
        let mut ret = Self {
            interval_x,
            interval_y,
            interval_z,
        };

        ret.pad_to_minimum();
        ret
    }

    fn pad_to_minimum(&mut self) {
        let min = f32::EPSILON as f64;
        if self.interval_x.size() < min { self.interval_x = self.interval_x.expand(min); }
        if self.interval_y.size() < min { self.interval_y = self.interval_y.expand(min); }
        if self.interval_z.size() < min { self.interval_z = self.interval_z.expand(min); }
    }


    pub fn from_points(pt1: &Vec3d, pt2: &Vec3d) -> Self {
        let interval_x = Interval { min: pt1.x().min(pt2.x()), max: pt1.x().max(pt2.x()) };
        let interval_y = Interval { min: pt1.y().min(pt2.y()), max: pt1.y().max(pt2.y()) };
        let interval_z = Interval { min: pt1.z().min(pt2.z()), max: pt1.z().max(pt2.z()) };
        Self::new(interval_x, interval_y, interval_z)
    }

    pub fn surrounding_box(box1: &AABB, box2: &AABB) -> Self {
        let interval_x = Interval::interval(&box1.interval_x, &box2.interval_x);
        let interval_y = Interval::interval(&box1.interval_y, &box2.interval_y);
        let interval_z = Interval::interval(&box1.interval_z, &box2.interval_z);
        Self::new(interval_x, interval_y, interval_z)
    }

    pub fn axis_interval(&self, axis: usize) -> Interval {
        match axis {
            0 => self.interval_x.clone(),
            1 => self.interval_y.clone(),
            2 => self.interval_z.clone(),
            _ => panic!("Invalid axis: {}", axis),
        }
    }

    pub fn hit(&self, ray: &Ray, interval: &Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let origin_axis = ray.origin[axis];
            let ray_dir = ray.direction[axis];

            if ray_dir.abs() < f64::EPSILON {
                // Ray is parallel to the axis. Check if the origin is within the interval.
                if origin_axis < ax.min || origin_axis > ax.max {
                    return false;
                }
                continue;
            }

            let adinv = 1.0 / ray_dir;

            let t0 = (ax.min - origin_axis) * adinv;
            let t1 = (ax.max - origin_axis) * adinv;

            let interval_hit: Interval = if t0 < t1 {
                Interval {
                    min: t0.max(interval.min),
                    max: t1.min(interval.max),
                }
            } else {
                Interval {
                    min: t1.max(interval.min),
                    max: t0.min(interval.max),
                }
            };

            if interval_hit.max <= interval_hit.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        let x_size = self.interval_x.size();
        let y_size = self.interval_y.size();
        let z_size = self.interval_z.size();

        if x_size > y_size && x_size > z_size {
            0
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    pub const EMPTY: AABB = AABB {
        interval_x: Interval::EMPTY,
        interval_y: Interval::EMPTY,
        interval_z: Interval::EMPTY,
    };

    pub const UNIVERSE: AABB = AABB {
        interval_x: Interval::UNIVERSE,
        interval_y: Interval::UNIVERSE,
        interval_z: Interval::UNIVERSE,
    };
}


#[cfg(test)]
mod test_aabb {
    use super::*;

    #[test]
    fn test_aabb_new() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );

        assert_eq!(aabb.interval_x, Interval { min: 1.0, max: 2.0 });
        assert_eq!(aabb.interval_y, Interval { min: 3.0, max: 4.0 });
        assert_eq!(aabb.interval_z, Interval { min: 5.0, max: 6.0 });
    }

    #[test]
    fn test_aabb_empty() {
        assert_eq!(AABB::EMPTY.interval_x, Interval::EMPTY);
        assert_eq!(AABB::EMPTY.interval_y, Interval::EMPTY);
        assert_eq!(AABB::EMPTY.interval_z, Interval::EMPTY);
    }

    #[test]
    fn test_aabb_from_points_1() {
        let aabb = AABB::from_points(
            &Vec3d::new(1.0, 2.0, 3.0),
            &Vec3d::new(4.0, 5.0, 6.0),
        );
        assert_eq!(aabb.interval_x, Interval { min: 1.0, max: 4.0 });
        assert_eq!(aabb.interval_y, Interval { min: 2.0, max: 5.0 });
        assert_eq!(aabb.interval_z, Interval { min: 3.0, max: 6.0 });
    }

    #[test]
    fn test_aabb_from_points_2() {
        let aabb = AABB::from_points(
            &Vec3d::new(4.0, 5.0, 6.0),
            &Vec3d::new(1.0, 2.0, 3.0),
        );
        assert_eq!(aabb.interval_x, Interval { min: 1.0, max: 4.0 });
        assert_eq!(aabb.interval_y, Interval { min: 2.0, max: 5.0 });
        assert_eq!(aabb.interval_z, Interval { min: 3.0, max: 6.0 });
    }

    #[test]
    fn test_aabb_surrounding_box_1() {
        let box1 = AABB::from_points(
            &Vec3d::new(1.0, 2.0, 3.0),
            &Vec3d::new(4.0, 5.0, 6.0),
        );
        let box2 = AABB::from_points(
            &Vec3d::new(0.0, 1.0, 2.0),
            &Vec3d::new(5.0, 6.0, 7.0),
        );
        let aabb = AABB::surrounding_box(&box1, &box2);
        assert_eq!(aabb.interval_x, Interval { min: 0.0, max: 5.0 });
        assert_eq!(aabb.interval_y, Interval { min: 1.0, max: 6.0 });
        assert_eq!(aabb.interval_z, Interval { min: 2.0, max: 7.0 });
    }

    #[test]
    fn test_aabb_surrounding_box_2() {
        let box1 = AABB::from_points(
            &Vec3d::new(11.0, 2.0, 3.0),
            &Vec3d::new(17.0, 15.0, 6.0),
        );
        let box2 = AABB::from_points(
            &Vec3d::new(0.5, 17.4, 21.0),
            &Vec3d::new(15.3, 46.0, 7.0),
        );
        let aabb = AABB::surrounding_box(&box2, &box1);
        assert_eq!(aabb.interval_x, Interval { min: 0.5, max: 17.0 });
        assert_eq!(aabb.interval_y, Interval { min: 2.0, max: 46.0 });
        assert_eq!(aabb.interval_z, Interval { min: 3.0, max: 21.0 });
    }

    #[test]
    fn test_aabb_axis_interval() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );
        assert_eq!(aabb.axis_interval(0), Interval { min: 1.0, max: 2.0 });
        assert_eq!(aabb.axis_interval(1), Interval { min: 3.0, max: 4.0 });
        assert_eq!(aabb.axis_interval(2), Interval { min: 5.0, max: 6.0 });
    }

    #[test]
    fn test_aabb_longest_axis_x() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 3.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );
        assert_eq!(aabb.longest_axis(), 0);
    }

    #[test]
    fn test_aabb_longest_axis_y() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 5.0 },
            Interval { min: 5.0, max: 6.0 },
        );
        assert_eq!(aabb.longest_axis(), 1);
    }

    #[test]
    fn test_aabb_longest_axis_z() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 7.0 },
        );
        assert_eq!(aabb.longest_axis(), 2);
    }

    #[test]
    fn test_aabb_pad_to_minimum() {
        let aabb = AABB::new(
            Interval { min: 0.0, max: 0.0 },
            Interval { min: 0.0, max: 0.0 },
            Interval { min: 0.0, max: 0.0 },
        );

        assert_ne!(aabb.interval_x.size(), 0.0);
        assert_ne!(aabb.interval_y.size(), 0.0);
        assert_ne!(aabb.interval_z.size(), 0.0);
    }

    #[test]
    fn test_aabb_hit_1() {
        let aabb = AABB::new(
            Interval { min: -1.0, max: 1.0 },
            Interval { min: -1.0, max: 1.0 },
            Interval { min: -1.0, max: 1.0 },
        );

        let ray = Ray::new(
            Vec3d::new(0.0, -5.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            0.0,
        );

        assert!(aabb.hit(&ray, &Interval { min: 0.0, max: 10.0 }));
        assert!(!aabb.hit(&ray, &Interval { min: 10.0, max: 20.0 }));
    }

    #[test]
    fn test_aabb_hit_2() {
        let aabb = AABB::new(
            Interval { min: -1.0, max: 1.0 },
            Interval { min: -1.0, max: 1.0 },
            Interval { min: -1.0, max: 1.0 },
        );

        let ray = Ray::new(
            Vec3d::new(2.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            0.0,
        );

        assert!(!aabb.hit(&ray, &Interval { min: 0.0, max: 10.0 }));
    }

    #[test]
    fn test_aabb_add_vec3d() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );

        let result = aabb + Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(result.interval_x, Interval { min: 2.0, max: 3.0 });
        assert_eq!(result.interval_y, Interval { min: 5.0, max: 6.0 });
        assert_eq!(result.interval_z, Interval { min: 8.0, max: 9.0 });
    }

    #[test]
    fn test_aabb_add_ref_vec3d() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );

        let result = aabb + &Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(result.interval_x, Interval { min: 2.0, max: 3.0 });
        assert_eq!(result.interval_y, Interval { min: 5.0, max: 6.0 });
        assert_eq!(result.interval_z, Interval { min: 8.0, max: 9.0 });
    }

    #[test]
    fn test_aabb_sub_vec3d() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );
        let result = aabb - Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(result.interval_x, Interval { min: 0.0, max: 1.0 });
        assert_eq!(result.interval_y, Interval { min: 1.0, max: 2.0 });
        assert_eq!(result.interval_z, Interval { min: 2.0, max: 3.0 });
    }

    #[test]
    fn test_aabb_sub_ref_vec3d() {
        let aabb = AABB::new(
            Interval { min: 1.0, max: 2.0 },
            Interval { min: 3.0, max: 4.0 },
            Interval { min: 5.0, max: 6.0 },
        );
        let result = aabb - &Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(result.interval_x, Interval { min: 0.0, max: 1.0 });
        assert_eq!(result.interval_y, Interval { min: 1.0, max: 2.0 });
        assert_eq!(result.interval_z, Interval { min: 2.0, max: 3.0 });
    }
}