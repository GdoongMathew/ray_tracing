use crate::vec3d::{Vec3d, Point3d};


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
///     0.0,
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
    pub origin: Point3d,
    pub direction: Vec3d,
    pub time: f64,
}

impl Ray {

    pub fn default() -> Self {
        Self { origin: Point3d::zero(), direction: Vec3d::zero(), time: 0.0}
    }

    pub fn new(origin: Point3d, direction: Vec3d, time: f64) -> Self {
        Self { origin, direction, time }
    }

    pub fn at(&self, t: f64) -> Point3d {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_ray_new() {
        let ray = Ray::new(
            Point3d::new(1.0, 2.0, 3.0),
            Vec3d::new(4.0, 5.0, 6.0),
            0.0,
        );
        assert_eq!(ray.origin, Point3d::new(1.0, 2.0, 3.0));
        assert_eq!(ray.direction, Vec3d::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(
            Point3d::new(1.0, 2.0, 3.0),
            Vec3d::new(4.0, 5.0, 6.0),
            0.0,
        );

        let t: f64 = 0.5;
        let result = ray.at(t);
        assert_eq!(result, Point3d::new(3.0, 4.5, 6.0));

        let t: f64 = 2.0;
        let result = ray.at(t);
        assert_eq!(result, Point3d::new(9.0, 12.0, 15.0));

        let t: f64 = 3.0;
        let result = ray.at(t);
        assert_eq!(result, Point3d::new(13.0, 17.0, 21.0));
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {

    /// Creates a new interval with the given two interval values.
    /// # Arguments
    /// * `interval_1` - The first interval value.
    /// * `interval_2` - The second interval value.
    /// # Returns
    /// The new interval.
    /// # Examples
    /// ```
    /// use ray_tracing::ray::Interval;
    /// let interval_1 = Interval{min: 1.0, max:2.0};
    /// let interval_2 = Interval{min: 3.0, max:4.0};
    /// let result = Interval::interval(&interval_1, &interval_2);
    /// assert_eq!(result.min, 1.0);
    /// assert_eq!(result.max, 4.0);
    /// ```
    pub fn interval(interval_1: &Self, interval_2: &Self) -> Self {
        Self {
            min: interval_1.min.min(interval_2.min),
            max: interval_1.max.max(interval_2.max),
        }
    }

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

    /// Returns the size of the interval.
    /// # Examples
    /// ```
    /// use ray_tracing::ray::Interval;
    /// let interval = Interval { min: 1.0, max: 2.0 };
    /// assert_eq!(interval.size(), 1.0);
    /// ```
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Expands the interval by a given amount.
    /// # Arguments
    /// * `t` - The amount to expand the interval by.
    /// # Returns
    /// The expanded interval.
    /// # Examples
    /// ```
    /// use ray_tracing::ray::Interval;
    /// let interval = Interval { min: 1.0, max: 2.0 };
    /// let result = interval.expand(0.5);
    /// assert_eq!(result.min, 0.75);
    /// assert_eq!(result.max, 2.25);
    /// ```
    pub fn expand(&self, t: f64) -> Interval {
        let delta = t * 0.5;
        Interval { min: self.min - delta, max: self.max + delta }
    }

    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };

    pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
}


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
        assert_eq!(Interval::EMPTY.min, f64::INFINITY);
        assert_eq!(Interval::EMPTY.max, f64::NEG_INFINITY);
    }

    #[test]
    fn test_interval_universe() {
        assert_eq!(Interval::UNIVERSE.min, f64::NEG_INFINITY);
        assert_eq!(Interval::UNIVERSE.max, f64::INFINITY);
    }

    #[test]
    fn test_interval_expand() {
        let interval = Interval { min: 1.0, max: 2.0 };
        let result = interval.expand(0.5);
        assert_eq!(result.min, 0.75);
        assert_eq!(result.max, 2.25);
    }

    #[test]
    fn test_interval_expand() {
        let interval = Interval { min: 0.0, max: 2.8 };
        let result = interval.expand(1.2);
        assert_eq!(result.min, -0.6);
        assert_eq!(result.max, 3.4);
    }
}
