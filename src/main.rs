use ray_tracing::vec3d::Vec3d;
use ray_tracing::ray::{Interval, Ray};
use ray_tracing::camera::Camera;
use ray_tracing::object::{HitRecord, Sphere, Hittable, HittableVec};
use ray_tracing::image::write_image;


fn ray_color<H: Hittable>(ray: &Ray, world: &H) -> Vec3d {

    let mut hit_record = HitRecord::new();
    if world.hit(ray, &Interval{min: 0.0, max: f64::INFINITY}, &mut hit_record) {
        return (hit_record.normal + Vec3d::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Vec3d::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3d::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let camera = Camera::new(
        Vec3d::new(0.0, 0.0, 0.0),
        1.0,
        16.0 / 9.0,
        1080,
        2.0,
    );

    let mut world = HittableVec::new();
    world.add(Box::new(Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0)));

    let image = camera.render(&world);

    write_image("output.png", &image, camera.resolution_width(), camera.resolution_height());
}