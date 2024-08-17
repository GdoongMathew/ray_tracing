pub mod hit;
pub mod sphere;
pub mod material;
mod aabb;

pub use hit::{HitRecord, Hittable, HittableVec};
pub use sphere::Sphere;
