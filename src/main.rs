use ray_tracing::vec3d::Vec3d;
use ray_tracing::camera::Camera;
use ray_tracing::object::{Sphere, HittableVec};
use ray_tracing::image::write_image;
use ray_tracing::object::material::{Material, Lambertian, Metal, Dielectric};



fn main() {
    let mut camera = Camera::new(
        Vec3d::new(0.0, 0.0, 0.0),
        1.0,
        16.0 / 9.0,
        1080,
        2.0,
    );

    camera.set_depth(50);

    let mut world = HittableVec::new();

    let ground = Material::Lambertian(Lambertian::new(Vec3d::new(0.8, 0.8, 0.0)));
    let center = Material::Lambertian(Lambertian::new(Vec3d::new(0.1, 0.2, 0.5)));
    let material_left = Material::Dielectric(Dielectric::new(1.0));
    // let material_left = Material::Metal(Metal::new(Vec3d::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Material::Metal(Metal::new(Vec3d::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0, ground)));
    world.add(Box::new(Sphere::new(Vec3d::new(0.0, 0.0, -1.2), 0.5, center)));
    world.add(Box::new(Sphere::new(Vec3d::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3d::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let image = camera.render(&world);

    write_image("output.png", &image, camera.resolution_width(), camera.resolution_height());
}