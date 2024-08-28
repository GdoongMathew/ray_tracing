use ray_tracing::object::BVHNode;
use ray_tracing::image::write_image;
use ray_tracing::scene;
use std::time::Instant;

fn main() {
    let (mut camera, world) = scene::quads();
    let world_ref: &'static BVHNode = Box::leak(Box::new(world));

    let now = Instant::now();
    let image = camera.render(world_ref);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    write_image("output.png", &image, camera.resolution_width(), camera.resolution_height());
}