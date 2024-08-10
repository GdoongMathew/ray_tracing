use ray_tracing::vec3d::Vec3d;
use ray_tracing::camera::Camera;
use ray_tracing::object::{Sphere, HittableVec};
use ray_tracing::image::write_image;



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