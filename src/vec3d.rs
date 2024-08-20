use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use rand::Rng;
use rand::distr::{Distribution, Standard};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3d {
    vector: [f64; 3],
}

/// Implementation of ``Vec3d``
///
/// This is a struct that represents a 3D vector with x, y, and z components.
/// Normally, this struct is used to represent points in 3D space, but it can also be used to
/// represent colors. In this particular library, it's used to represent vectors or points in
/// 3D space, and can be used for various calculations.
/// Most of this struct's implementation follows the same pattern in the website
/// [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
///
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// assert_eq!(vec.x(), 1.0);
/// assert_eq!(vec.y(), 2.0);
/// assert_eq!(vec.z(), 3.0);
/// ```
impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vector: [x, y, z],
        }
    }

    /// Returns a Vec3d with all components set to zero
    /// # Examples
    /// ```
    /// use ray_tracing::vec3d::Vec3d;
    /// let vec = Vec3d::zero();
    /// assert_eq!(vec, Vec3d::new(0.0, 0.0, 0.0));
    /// ```
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 { self.vector[0] }

    pub fn y(&self) -> f64 { self.vector[1] }

    pub fn z(&self) -> f64 { self.vector[2] }

    /// Returns the length of the vector
    /// # Examples
    /// ```
    /// use ray_tracing::vec3d::Vec3d;
    /// let vec = Vec3d::new(1.0, 2.0, 3.0);
    /// assert_eq!(vec.length(), 3.7416573867739413);
    /// ```
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the squared length of the vector
    /// # Examples
    /// ```
    /// use ray_tracing::vec3d::Vec3d;
    /// let vec = Vec3d::new(1.0, 2.0, 3.0);
    /// assert_eq!(vec.length_squared(), 14.0);
    /// ```
    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    /// Returns the unit vector of the vector
    /// # Examples
    /// ```
    /// use ray_tracing::vec3d::Vec3d;
    /// let vec = Vec3d::new(1.0, 2.0, 3.0);
    /// let result = vec.unit_vector();
    /// assert_eq!(result, Vec3d::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732));
    /// ```
    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        rng.random()
    }

    pub fn gen_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3d::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3d::gen_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3d) -> Self {
        let in_unit_sphere = Vec3d::random_in_unit_sphere();
        if dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        self.x().abs() < f64::EPSILON &&
            self.y().abs() < f64::EPSILON &&
            self.z().abs() < f64::EPSILON
    }

    #[inline]
    fn zip_with(
        &self,
        other: &Vec3d,
        mut f: impl FnMut(f64, f64) -> f64,
    ) -> Self {
        Vec3d::new(
            f(self.x(), other.x()),
            f(self.y(), other.y()),
            f(self.z(), other.z()),
        )
    }

    #[inline]
    pub fn reduce(&self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self.x(), self.y()), self.z())
    }

    #[inline]
    pub fn map(&self, f: impl Fn(f64) -> f64) -> Self {
        Vec3d::new(f(self.x()), f(self.y()), f(self.z()))
    }
}


/// Implementation of ``rand::distr::Distribution`` for ``Vec3d``
/// # Examples
/// ```
/// use rand::Rng;
/// use rand::distr::Distribution;
/// use ray_tracing::vec3d::Vec3d;
/// let mut rng = rand::thread_rng();
/// let vec: Vec3d = rng.random();
/// ```
impl Distribution<Vec3d> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3d {
        let (x, y, z) = rng.random::<(f64, f64, f64)>();
        Vec3d::new(x, y, z)
    }
}


/// Implementation of ``std::fmt::Display`` for ``Vec3d``
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// assert_eq!(format!("{}", vec), "Vec3d[1, 2, 3]");
/// ```
impl std::fmt::Display for Vec3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3d[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}


/// The dot product of two Vec3d vectors
/// # Examples
/// ```
/// use ray_tracing::vec3d::{Vec3d, dot};
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// let result = dot(&vec, &vec2);
/// assert_eq!(result, 32.0);
/// ```
#[inline]
pub fn dot(v1: &Vec3d, v2: &Vec3d) -> f64 {
    v1.zip_with(v2, Mul::mul).reduce(Add::add)
}


/// The distance between two vec3d vectors
/// # Examples
/// ```
/// use ray_tracing::vec3d::{Vec3d, distance};
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 6.0, 3.0);
/// let result = distance(&vec, &vec2);
/// assert_eq!(result, 5.0);
/// ```
#[inline]
pub fn distance<'a>(v1: &'a Vec3d, v2: &'a Vec3d) -> f64 {
    (*v1 - *v2).length()
}


/// The cross product of two Vec3d vectors
/// # Examples
/// ```
/// use ray_tracing::vec3d::{Vec3d, cross};
/// let vec = Vec3d::new(1.0, 0.0, 0.0);
/// let vec2 = Vec3d::new(0.0, 1.0, 0.0);
/// let result = cross(&vec, &vec2);
/// assert_eq!(result, Vec3d::new(0.0, 0.0, 1.0));
/// ```
#[inline]
pub fn cross(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    Vec3d::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        v1.z() * v2.x() - v1.x() * v2.z(),
        v1.x() * v2.y() - v1.y() * v2.x(),
    )
}


impl Neg for Vec3d {
    type Output = Self;

    /// Returns the negation of the vector
    /// # Examples
    /// ```
    /// use ray_tracing::vec3d::Vec3d;
    /// let vec = Vec3d::new(1.0, 2.0, 3.0);
    /// let result = -vec;
    /// assert_eq!(result, Vec3d::new(-1.0, -2.0, -3.0));
    /// ```
    #[inline]
    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}


/// Addition overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// let result = vec + vec2;
/// assert_eq!(result, Vec3d::new(5.0, 7.0, 9.0));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec + 2.0;
/// assert_eq!(result, Vec3d::new(3.0, 4.0, 5.0));
/// ```
impl Add<Vec3d> for Vec3d {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.zip_with(&rhs, Add::add)
    }
}

/// Overloading for adding a Vec3d to a scalar
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec + 2.0;
/// assert_eq!(result, Vec3d::new(3.0, 4.0, 5.0));
/// ```
impl Add<f64> for Vec3d {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        self.map(|x| x + rhs)
    }
}

/// Overloading for adding a scalar to a Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = 2.0 + vec;
/// assert_eq!(result, Vec3d::new(3.0, 4.0, 5.0));
/// ```
impl Add<Vec3d> for f64 {
    type Output = Vec3d;

    #[inline]
    fn add(self, rhs: Vec3d) -> Self::Output {
        rhs + self
    }
}

/// AddAssign overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// vec += vec2;
/// assert_eq!(vec, Vec3d::new(5.0, 7.0, 9.0));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// vec += 2.0;
/// assert_eq!(vec, Vec3d::new(3.0, 4.0, 5.0));
/// ```
impl AddAssign<Vec3d> for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.vector[0] += rhs.x();
        self.vector[1] += rhs.y();
        self.vector[2] += rhs.z();
    }
}

impl AddAssign<f64> for Vec3d {
    fn add_assign(&mut self, rhs: f64) {
        self.vector[0] += rhs;
        self.vector[1] += rhs;
        self.vector[2] += rhs;
    }
}


/// Subtraction overloading for Vec3d
///
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// let result = vec - vec2;
/// assert_eq!(result, Vec3d::new(-3.0, -3.0, -3.0));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec - 2.0;
/// assert_eq!(result, Vec3d::new(-1.0, 0.0, 1.0));
/// ```
impl Sub<Vec3d> for Vec3d {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_with(&rhs, Sub::sub)
    }
}


/// Overloading for subtracting a scalar from a Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec - 2.0;
/// assert_eq!(result, Vec3d::new(-1.0, 0.0, 1.0));
/// ```
impl Sub<f64> for Vec3d {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        self.map(|x| x - rhs)
    }
}


/// Overloading for subtracting a Vec3d from a scalar
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = 2.0 - vec;
/// assert_eq!(result, Vec3d::new(1.0, 0.0, -1.0));
/// ```
impl Sub<Vec3d> for f64 {
    type Output = Vec3d;
    #[inline]
    fn sub(self, rhs: Vec3d) -> Self::Output {
        -rhs + self
    }
}

/// SubAssign overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// vec -= vec2;
/// assert_eq!(vec, Vec3d::new(-3.0, -3.0, -3.0));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// vec -= 2.0;
/// assert_eq!(vec, Vec3d::new(-1.0, 0.0, 1.0));
/// ```
impl SubAssign<Vec3d> for Vec3d {
    fn sub_assign(&mut self, rhs: Self) {
        self.vector[0] -= rhs.x();
        self.vector[1] -= rhs.y();
        self.vector[2] -= rhs.z();
    }
}

impl SubAssign<f64> for Vec3d {
    fn sub_assign(&mut self, rhs: f64) {
        self.vector[0] -= rhs;
        self.vector[1] -= rhs;
        self.vector[2] -= rhs;
    }
}


/// Multiplication overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// let result = vec * vec2;
/// assert_eq!(result, Vec3d::new(4.0, 10.0, 18.0));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec * 2.0;
/// assert_eq!(result, Vec3d::new(2.0, 4.0, 6.0));
/// ```
impl Mul<Vec3d> for Vec3d {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_with(&rhs, Mul::mul)
    }
}

/// Overloading for multiplying a Vec3d with a scalar
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec * 2.0;
/// assert_eq!(result, Vec3d::new(2.0, 4.0, 6.0));
/// ```
impl Mul<f64> for Vec3d {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|x| x * rhs)
    }
}


/// Overloading for multiplying a scalar with a Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = 2.0 * vec;
/// assert_eq!(result, Vec3d::new(2.0, 4.0, 6.0));
/// ```
impl Mul<Vec3d> for f64 {
    type Output = Vec3d;

    #[inline]
    fn mul(self, rhs: Vec3d) -> Self::Output { rhs * self }
}


/// MulAssign overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// vec *= vec2;
/// assert_eq!(vec, Vec3d::new(4.0, 10.0, 18.0));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// vec *= 2.0;
/// assert_eq!(vec, Vec3d::new(2.0, 4.0, 6.0));
/// ```
impl MulAssign<Vec3d> for Vec3d {
    fn mul_assign(&mut self, rhs: Self) {
        self.vector[0] *= rhs.x();
        self.vector[1] *= rhs.y();
        self.vector[2] *= rhs.z();
    }
}

impl MulAssign<f64> for Vec3d {
    fn mul_assign(&mut self, rhs: f64) {
        self.vector[0] *= rhs;
        self.vector[1] *= rhs;
        self.vector[2] *= rhs;
    }
}


/// Division overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// let result = vec / vec2;
/// assert_eq!(result, Vec3d::new(0.25, 0.4, 0.5));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let vec = Vec3d::new(1.0, 2.0, 3.0);
/// let result = vec / 2.0;
/// assert_eq!(result, Vec3d::new(0.5, 1.0, 1.5));
/// ```
impl Div<Vec3d> for Vec3d {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.zip_with(&rhs, Div::div)
    }
}

impl Div<f64> for Vec3d {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output { self * (1.0 / rhs) }
}


/// DivAssign overloading for Vec3d
/// # Examples
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// let vec2 = Vec3d::new(4.0, 5.0, 6.0);
/// vec /= vec2;
/// assert_eq!(vec, Vec3d::new(0.25, 0.4, 0.5));
/// ```
/// ```
/// use ray_tracing::vec3d::Vec3d;
/// let mut vec = Vec3d::new(1.0, 2.0, 3.0);
/// vec /= 2.0;
/// assert_eq!(vec, Vec3d::new(0.5, 1.0, 1.5));
/// ```
impl DivAssign<Vec3d> for Vec3d {
    fn div_assign(&mut self, rhs: Self) {
        self.vector[0] /= rhs.x();
        self.vector[1] /= rhs.y();
        self.vector[2] /= rhs.z();
    }
}

impl DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

#[cfg(test)]
mod vec3d_tests {
    use super::*;

    #[test]
    fn test_vec3d_new() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }

    #[test]
    fn test_vec3d_length() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(vec.length(), 3.7416573867739413);
    }

    #[test]
    fn test_vec3d_length_squared() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(vec.length_squared(), 14.0);
    }

    #[test]
    fn test_vec3d_unit_vector() {
        let vec = Vec3d::new(10.0, 0.0, 0.0);
        let result = vec.unit_vector();
        assert_eq!(result, Vec3d::new(1.0, 0.0, 0.0));

        let vec = Vec3d::new(0.0, 10.0, 0.0);
        let result = vec.unit_vector();
        assert_eq!(result, Vec3d::new(0.0, 1.0, 0.0));

        let vec = Vec3d::new(0.0, 0.0, 10.0);
        let result = vec.unit_vector();
        assert_eq!(result, Vec3d::new(0.0, 0.0, 1.0));

        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = vec.unit_vector();
        assert_eq!(result, Vec3d::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732));
    }

    #[test]
    fn test_vec3d_dot() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        let result = dot(&vec, &vec2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vec3d_distance() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 6.0, 3.0);
        let result = distance(&vec, &vec2);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_vec3d_cross() {
        let vec = Vec3d::new(1.0, 0.0, 0.0);
        let vec2 = Vec3d::new(0.0, 1.0, 0.0);
        let result = cross(&vec, &vec2);
        assert_eq!(result, Vec3d::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vec3d_display() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", vec), "Vec3d[1, 2, 3]");
    }

    #[test]
    fn test_vec3d_neg() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = -vec;
        assert_eq!(result, Vec3d::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vec3d_add_vec3d() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        let result = vec + vec2;
        assert_eq!(result, Vec3d::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec3d_add_f64() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = vec + 2.0;
        assert_eq!(result, Vec3d::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_vec3d_add_assign_vec3d() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        vec += vec2;
        assert_eq!(vec, Vec3d::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec3d_add_assign_f64() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        vec += 2.0;
        assert_eq!(vec, Vec3d::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_vec3d_sub_vec3d() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        let result = vec - vec2;
        assert_eq!(result, Vec3d::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_vec3d_sub_f64() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = vec - 2.0;
        assert_eq!(result, Vec3d::new(-1.0, 0.0, 1.0));
    }

    #[test]
    fn test_vec3d_sub_assign_vec3d() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        vec -= vec2;
        assert_eq!(vec, Vec3d::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_vec3d_sub_assign_f64() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        vec -= 2.0;
        assert_eq!(vec, Vec3d::new(-1.0, 0.0, 1.0));
    }

    #[test]
    fn test_vec3d_mul_vec3d() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        let result = vec * vec2;
        assert_eq!(result, Vec3d::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_vec3d_mul_f64() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = vec * 2.0;
        assert_eq!(result, Vec3d::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_f64_mul_vec3d() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = 2.0 * vec;
        assert_eq!(result, Vec3d::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3d_mul_assign_vec3d() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        vec *= vec2;
        assert_eq!(vec, Vec3d::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_vec3d_mul_assign_f64() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        vec *= 2.0;
        assert_eq!(vec, Vec3d::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3d_div_vec3d() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        let result = vec / vec2;
        assert_eq!(result, Vec3d::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn test_vec3d_div_f64() {
        let vec = Vec3d::new(1.0, 2.0, 3.0);
        let result = vec / 2.0;
        assert_eq!(result, Vec3d::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_vec3d_div_assign_vec3d() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        let vec2 = Vec3d::new(4.0, 5.0, 6.0);
        vec /= vec2;
        assert_eq!(vec, Vec3d::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn test_vec3d_div_assign_f64() {
        let mut vec = Vec3d::new(1.0, 2.0, 3.0);
        vec /= 2.0;
        assert_eq!(vec, Vec3d::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn test_vec3d_random_fn() {
        let vec = Vec3d::random();
        assert_eq!(vec.x() >= 0.0 && vec.x() <= 1.0, true);
        assert_eq!(vec.y() >= 0.0 && vec.y() <= 1.0, true);
        assert_eq!(vec.z() >= 0.0 && vec.z() <= 1.0, true);
    }

    #[test]
    fn test_random_vec3d() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let vec: Vec3d = rng.random();

        assert_eq!(vec.x() >= 0.0 && vec.x() <= 1.0, true);
        assert_eq!(vec.y() >= 0.0 && vec.y() <= 1.0, true);
        assert_eq!(vec.z() >= 0.0 && vec.z() <= 1.0, true);
    }

    #[test]
    fn test_vec3d_gen_range_0_1() {
        let vec = Vec3d::gen_range(0.0, 1.0);
        assert_eq!(vec.x() >= 0.0 && vec.x() <= 1.0, true);
        assert_eq!(vec.y() >= 0.0 && vec.y() <= 1.0, true);
        assert_eq!(vec.z() >= 0.0 && vec.z() <= 1.0, true);
    }

    #[test]
    fn test_vec3d_gen_range_5_10() {
        let vec = Vec3d::gen_range(5.0, 10.0);
        assert_eq!(vec.x() >= 5.0 && vec.x() <= 10.0, true);
        assert_eq!(vec.y() >= 5.0 && vec.y() <= 10.0, true);
        assert_eq!(vec.z() >= 5.0 && vec.z() <= 10.0, true);
    }
}
