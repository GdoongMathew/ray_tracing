use std::sync::Arc;
use crate::object::material::Material;
use crate::vec3d::{Point3d, Vec3d};
use crate::object::{HittableVec, Quad};

pub fn bbox(a: Point3d, b: Point3d, material: Material) -> HittableVec {
    let mut sides = HittableVec::new();

    let min = Point3d::new(
        a.x().min(b.x()),
        a.y().min(b.y()),
        a.z().min(b.z()),
    );

    let max = Point3d::new(
        a.x().max(b.x()),
        a.y().max(b.y()),
        a.z().max(b.z()),
    );

    let dx = Vec3d::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3d::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3d::new(0.0, 0.0, max.z() - min.z());

    sides.add(
        Arc::new(Box::new(Quad::new(
            Point3d::new(min.x(), min.y(), max.z()),
            dx, dy, material.clone(),
        )))
    );
    sides.add(
        Arc::new(Box::new(Quad::new(
            Point3d::new(max.x(), min.y(), max.z()),
            -dz, dy, material.clone(),
        )))
    );
    sides.add(
        Arc::new(Box::new(Quad::new(
            Point3d::new(max.x(), min.y(), min.z()),
            -dx, dy, material.clone(),
        )))
    );
    sides.add(
        Arc::new(Box::new(Quad::new(
            Point3d::new(min.x(), min.y(), min.z()),
            dz, dy, material.clone(),
        )))
    );
    sides.add(
        Arc::new(Box::new(Quad::new(
            Point3d::new(min.x(), max.y(), max.z()),
            dx, -dz, material.clone(),
        )))
    );
    sides.add(
        Arc::new(Box::new(Quad::new(
            Point3d::new(min.x(), min.y(), min.z()),
            dx, dz, material.clone(),
        )))
    );
    sides
}