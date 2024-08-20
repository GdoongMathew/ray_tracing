use ray_tracing::vec3d::Vec3d;
use ray_tracing::camera::Camera;
use ray_tracing::object::{Sphere, HittableVec, BVHNode};
use ray_tracing::image::write_image;
use ray_tracing::object::material::{Material, Lambertian, Metal, Dielectric};
use ray_tracing::object::texture::Checker;

use rand::Rng;
use std::time::Instant;
use std::sync::Arc;

fn main() {
    let mut rng = rand::thread_rng();
    let mut camera = Camera::new();

    camera.set_depth(50);
    camera.set_aspect_ratio(16.0 / 9.0);
    camera.set_resolution_width(1080);
    camera.set_samples_per_pixel(500);
    camera.set_v_fov(20.0);

    camera.set_look_from(Vec3d::new(13.0, 2.0, 3.0));
    camera.set_look_at(Vec3d::new(0.0, 0.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));

    camera.set_defocus_angle(0.6);
    camera.set_focus_dist(10.0);

    let mut world = HittableVec::new();

    let checker = Checker::from_color(
        Vec3d::new(0.2, 0.3, 0.1),
        Vec3d::new(0.9, 0.9, 0.9),
        0.32,
    );

    let ground = Material::Lambertian(Lambertian::from_texture(Box::new(checker)));
    world.add(
        Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, ground))));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = rand::random::<f64>();
    //         let center = Vec3d::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
    //         if (center - Vec3d::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             let sphere_material: Material;
    //             if choose_mat < 0.8 {
    //                 let albedo = Vec3d::random() * Vec3d::random();
    //                 sphere_material = Material::Lambertian(Lambertian::new(albedo));
    //                 let center2 = center + Vec3d::new(0.0, rng.gen_range(0.0..0.5), 0.0);
    //                 world.add(Arc::new(Box::new(Sphere::moving_sphere(center, center2, 0.2, sphere_material))));
    //             } else if choose_mat < 0.95 {
    //                 let albedo = Vec3d::gen_range(0.5, 1.0);
    //                 let fuzz = rand::random::<f64>() * 0.5;
    //                 sphere_material = Material::Metal(Metal::new(albedo, fuzz));
    //                 world.add(Arc::new(Box::new(Sphere::static_sphere(center, 0.2, sphere_material))));
    //             } else {
    //                 sphere_material = Material::Dielectric(Dielectric::new(1.5));
    //                 world.add(Arc::new(Box::new(Sphere::static_sphere(center, 0.2, sphere_material))));
    //             }
    //         }
    //     }
    // }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(0.0, 1.0, 0.0), 1.0, material1))));

    let material2 = Material::Lambertian(Lambertian::new(Vec3d::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(-4.0, 1.0, 0.0), 1.0, material2))));

    let material3 = Material::Metal(Metal::new(Vec3d::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(4.0, 1.0, 0.0), 1.0, material3))));

    // let world_ref: &'static HittableVec = Box::leak(Box::new(world));
    let world = BVHNode::from_hittable_vec(Arc::new(world));
    let world_ref: &'static BVHNode = Box::leak(Box::new(world));

    let now = Instant::now();
    let image = camera.render(world_ref);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);

    write_image("output.png", &image, camera.resolution_width(), camera.resolution_height());
}