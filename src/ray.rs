use crate::vec3d::Vec3d;


/// A ray is a line that starts at a point and goes in a direction.
/// # Fields
/// * `origin` - The starting point of the ray.
/// * `direction` - The direction of the ray.
/// # Examples
/// ```
/// use ray_tracing::ray::Ray;
/// use ray_tracing::vec3d::Vec3d;
/// let ray = Ray::new(
///     Vec3d::new(1.0, 2.0, 3.0),
///     Vec3d::new(4.0, 5.0, 6.0),
/// );
/// assert_eq!(ray.origin, Vec3d::new(1.0, 2.0, 3.0));
/// assert_eq!(ray.direction, Vec3d::new(4.0, 5.0, 6.0));
///
/// let t: f64 = 0.5;
/// let result = ray.at(t);
/// assert_eq!(result, Vec3d::new(3.0, 4.5, 6.0));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d,
}

impl Ray {

    pub fn default() -> Self {
        Self { origin: Vec3d::zero(), direction: Vec3d::zero() }
    }

    pub fn new(origin: Vec3d, direction: Vec3d) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3d {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_ray_new() {
        let ray = Ray::new(
            Vec3d::new(1.0, 2.0, 3.0),
            Vec3d::new(4.0, 5.0, 6.0),
        );
        assert_eq!(ray.origin, Vec3d::new(1.0, 2.0, 3.0));
        assert_eq!(ray.direction, Vec3d::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(
            Vec3d::new(1.0, 2.0, 3.0),
            Vec3d::new(4.0, 5.0, 6.0),
        );

        let t: f64 = 0.5;
        let result = ray.at(t);
        assert_eq!(result, Vec3d::new(3.0, 4.5, 6.0));

        let t: f64 = 2.0;
        let result = ray.at(t);
        assert_eq!(result, Vec3d::new(9.0, 12.0, 15.0));

        let t: f64 = 3.0;
        let result = ray.at(t);
        assert_eq!(result, Vec3d::new(13.0, 17.0, 21.0));
    }
}


/// An interval is a range of values between a minimum and maximum.
/// # Fields
/// * `min` - The minimum value of the interval.
/// * `max` - The maximum value of the interval.
/// # Examples
/// ```
/// use ray_tracing::ray::Interval;
/// let interval = Interval { min: 1.0, max: 2.0 };
/// assert_eq!(interval.min, 1.0);
/// assert_eq!(interval.max, 2.0);
/// ```
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }

    /// Clamps a value to the interval.
    /// # Arguments
    /// * `t` - The value to clamp.
    /// # Returns
    /// The clamped value.
    pub fn clamp(&self, t: f64) -> f64 {
        if t < self.min { self.min } else if t > self.max { self.max } else { t }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}

pub static EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
pub static UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };


#[cfg(test)]
mod test_interval {
    use super::*;

    #[test]
    fn test_interval_contains() {
        let interval = Interval { min: 1.0, max: 2.0 };
        assert_eq!(interval.contains(0.9), false);
        assert_eq!(interval.contains(1.0), true);
        assert_eq!(interval.contains(1.5), true);
        assert_eq!(interval.contains(2.0), true);
        assert_eq!(interval.contains(2.1), false);
    }

    #[test]
    fn test_interval_surrounds() {
        let interval = Interval { min: 1.0, max: 2.0 };
        assert_eq!(interval.surrounds(0.9), false);
        assert_eq!(interval.surrounds(1.0), false);
        assert_eq!(interval.surrounds(1.5), true);
        assert_eq!(interval.surrounds(2.0), false);
        assert_eq!(interval.surrounds(2.1), false);
    }

    #[test]
    fn test_interval_clamp() {
        let interval = Interval { min: 1.0, max: 2.0 };
        assert_eq!(interval.clamp(0.9), 1.0);
        assert_eq!(interval.clamp(1.0), 1.0);
        assert_eq!(interval.clamp(1.5), 1.5);
        assert_eq!(interval.clamp(2.0), 2.0);
        assert_eq!(interval.clamp(2.1), 2.0);
    }

    #[test]
    fn test_interval_size() {
        let interval = Interval { min: 1.0, max: 2.0 };
        assert_eq!(interval.size(), 1.0);
    }

    #[test]
    fn test_interval_empty() {
        assert_eq!(EMPTY.min, f64::INFINITY);
        assert_eq!(EMPTY.max, f64::NEG_INFINITY);
    }

    #[test]
    fn test_interval_universe() {
        assert_eq!(UNIVERSE.min, f64::NEG_INFINITY);
        assert_eq!(UNIVERSE.max, f64::INFINITY);
    }
}
