use ray_tracing::vec3d::Vec3d;
use ray_tracing::camera::Camera;
use ray_tracing::object::BVHNode;
use ray_tracing::image::write_image;
use ray_tracing::scene;
use std::time::Instant;

fn main() {
    let mut camera = Camera::new();

    camera.set_depth(50);
    camera.set_aspect_ratio(16.0 / 9.0);
    camera.set_resolution_width(400);
    camera.set_samples_per_pixel(50);
    camera.set_v_fov(20.0);

    camera.set_look_from(Vec3d::new(13.0, 2.0, 3.0));
    camera.set_look_at(Vec3d::new(0.0, 0.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));

    camera.set_defocus_angle(0.6);
    camera.set_focus_dist(10.0);

    let world: BVHNode = scene::bouncing_balls();
    let world_ref: &'static BVHNode = Box::leak(Box::new(world));

    let now = Instant::now();
    let image = camera.render(world_ref);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    write_image("output.png", &image, camera.resolution_width(), camera.resolution_height());
}