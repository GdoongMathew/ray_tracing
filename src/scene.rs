use std::sync::Arc;
use crate::object::{BVHNode, HittableVec, Sphere, Quad};
use crate::object::material::{Dielectric, Lambertian, Material, Metal};
use crate::object::texture::{Texture, Checker, ImageTexture, PerlinTexture, SolidColor};
use crate::vec3d::Vec3d;
use rand::Rng;

pub fn bouncing_balls() -> BVHNode {
    let mut rng = rand::thread_rng();
    let mut world = HittableVec::new();

    let checker: Arc<Box<dyn Texture>> = Arc::new(Box::new(Checker::from_color(
        Vec3d::new(0.2, 0.3, 0.1),
        Vec3d::new(0.9, 0.9, 0.9),
        0.32,
    )));

    let ground = Material::Lambertian(Lambertian::from_texture(checker.clone()));
    world.add(
        Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, ground))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Vec3d::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
            if (center - Vec3d::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;
                if choose_mat < 0.8 {
                    let albedo = Vec3d::random() * Vec3d::random();
                    sphere_material = Material::Lambertian(Lambertian::new(albedo));
                    let center2 = center + Vec3d::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Box::new(Sphere::moving_sphere(center, center2, 0.2, sphere_material))));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3d::gen_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    sphere_material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Box::new(Sphere::static_sphere(center, 0.2, sphere_material))));
                } else {
                    sphere_material = Material::Dielectric(Dielectric::new(1.5));
                    world.add(Arc::new(Box::new(Sphere::static_sphere(center, 0.2, sphere_material))));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(0.0, 1.0, 0.0), 1.0, material1))));

    let material2 = Material::Lambertian(Lambertian::new(Vec3d::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(-4.0, 1.0, 0.0), 1.0, material2))));

    let material3 = Material::Metal(Metal::new(Vec3d::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Box::new(Sphere::static_sphere(Vec3d::new(4.0, 1.0, 0.0), 1.0, material3))));

    BVHNode::from_hittable_vec(Arc::new(world))
}


pub fn checkered_spheres() -> BVHNode {
    let mut world = HittableVec::new();

    let checker: Box<dyn Texture> = Box::new(Checker::from_color(
        Vec3d::new(0.2, 0.3, 0.1),
        Vec3d::new(0.9, 0.9, 0.9),
        0.32,
    ));

    let arc_checker = Arc::new(checker);

    world.add(
        Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(0.0, -10.0, 0.0),
            10.0,
            Material::Lambertian(Lambertian::from_texture(arc_checker.clone())))))
    );

    world.add(
        Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(0.0, 10.0, 0.0),
            10.0,
            Material::Lambertian(Lambertian::from_texture(arc_checker.clone())))))
    );

    BVHNode::from_hittable_vec(Arc::new(world))
}


pub fn earth() -> BVHNode {
    let mut world = HittableVec::new();

    let image_file = "./misc/earthmap.png".to_string();
    let earth_texture: Arc<Box<dyn Texture>> = Arc::new(Box::new(ImageTexture::new(&image_file)));

    world.add(
        Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(0.0, 0.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::from_texture(earth_texture)))
        )));
    BVHNode::from_hittable_vec(Arc::new(world))
}


pub fn perlin_sphere() -> BVHNode {
    let mut world = HittableVec::new();

    let perlin_texture: Arc<Box<dyn Texture>> = Arc::new(Box::new(PerlinTexture::new(4.0)));
    world.add(
        Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::Lambertian(Lambertian::from_texture(perlin_texture.clone())),
        )))
    );
    world.add(
        Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::new(0.0, 2.0, 0.0),
            2.0,
            Material::Lambertian(Lambertian::from_texture(perlin_texture.clone())),
        )))
    );

    BVHNode::from_hittable_vec(Arc::new(world))
}


pub fn quads() -> HittableVec {
    let mut world = HittableVec::new();

    // Material
    let left_red = Material::Lambertian(
        Lambertian::from_texture(
            Arc::new(Box::new(SolidColor::new(
                Vec3d::new(1.0, 0.2, 0.2))))
        )
    );

    let back_green = Material::Lambertian(
        Lambertian::from_texture(
            Arc::new(Box::new(SolidColor::new(
                Vec3d::new(0.2, 1.0, 0.2)
            )))
        )
    );

    let right_blue = Material::Lambertian(
        Lambertian::from_texture(
            Arc::new(Box::new(SolidColor::new(
                Vec3d::new(0.2, 0.2, 1.0)
            )))
        )
    );

    let upper_orange = Material::Lambertian(
        Lambertian::from_texture(
            Arc::new(Box::new(SolidColor::new(
                Vec3d::new(1.0, 0.5, 0.0)
            )))
        )
    );

    let lower_teal = Material::Lambertian(
        Lambertian::from_texture(
            Arc::new(Box::new(SolidColor::new(
                Vec3d::new(0.2, 0.8, 0.8)
            )))
        )
    );

    // Quads
    world.add(Arc::new(Box::new(
        Quad::new(
            Vec3d::new(-3.0, -2.0, 5.0),
            Vec3d::new(0.0, 0.0, -4.0),
            Vec3d::new(0.0, 4.0, 0.0),
            left_red,
        )
    )));

    world.add(Arc::new(Box::new(
        Quad::new(
            Vec3d::new(-2.0, -2.0, 0.0),
            Vec3d::new(4.0, 0.0, 0.0),
            Vec3d::new(0.0, 4.0, 0.0),
            back_green,
        )
    )));

    world.add(Arc::new(Box::new(
        Quad::new(
            Vec3d::new(3.0, -2.0, 1.0),
            Vec3d::new(0.0, 0.0, 4.0),
            Vec3d::new(0.0, 4.0, 0.0),
            right_blue,
        )
    )));
    world.add(Arc::new(Box::new(
        Quad::new(
            Vec3d::new(-2.0, 3.0, 1.0),
            Vec3d::new(4.0, 0.0, 0.0),
            Vec3d::new(0.0, 0.0, 4.0),
            upper_orange,
        )
    )));

    world.add(Arc::new(Box::new(
        Quad::new(
            Vec3d::new(-2.0, -3.0, 5.0),
            Vec3d::new(4.0, 0.0, 0.0),
            Vec3d::new(0.0, 0.0, -4.0),
            lower_teal,
        )
    )));
    world
}