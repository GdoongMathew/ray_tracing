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