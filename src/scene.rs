#[forbid(unsafe_code)]

use std::sync::Arc;
use crate::object::{BVHNode, HittableVec, Sphere, Quad, bbox, Hittable, Translate, RotateY, Medium};
use crate::object::material::{Dielectric, Lambertian, Material, Metal, Light};
use crate::object::texture::{Texture, Checker, ImageTexture, PerlinTexture, SolidColor};
use crate::vec3d::{Vec3d, Color, Point3d};
use rand::Rng;
use crate::camera::Camera;

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


pub fn perlin_sphere() -> (Camera, BVHNode) {
    let mut camera = Camera::new();
    camera.set_aspect_ratio(16.0 / 9.0);
    camera.set_resolution_width(400);
    camera.set_samples_per_pixel(100);
    camera.set_depth(50);

    camera.set_v_fov(20.0);
    camera.set_look_from(Vec3d::new(13.0, 2.0, 3.0));
    camera.set_look_at(Vec3d::new(0.0, 0.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));

    camera.set_defocus_angle(0.0);
    camera.set_background_color(
        Color::new(0.7, 0.8, 1.0)
    );

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

    (camera, BVHNode::from_hittable_vec(Arc::new(world)))
}


pub fn quads() -> (Camera, BVHNode) {
    let mut camera = Camera::new();

    camera.set_depth(50);
    camera.set_aspect_ratio(1.0);
    camera.set_resolution_width(400);
    camera.set_samples_per_pixel(100);
    camera.set_v_fov(80.0);

    camera.set_background_color(
        Color::new(0.7, 0.8, 1.0)
    );

    camera.set_look_from(Vec3d::new(0.0, 0.0, 9.0));
    camera.set_look_at(Vec3d::new(0.0, 0.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));

    camera.set_defocus_angle(0.0);


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
    (camera, BVHNode::from_hittable_vec(Arc::new(world)))
}


pub fn simple_light() -> (Camera, BVHNode) {
    let mut camera = Camera::new();

    camera.set_depth(50);
    camera.set_aspect_ratio(16.0 / 9.0);
    camera.set_resolution_width(400);
    camera.set_samples_per_pixel(100);

    camera.set_v_fov(20.0);
    camera.set_look_from(Vec3d::new(26.0, 3.0, 6.0));
    camera.set_look_at(Vec3d::new(0.0, 2.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));

    camera.set_background_color(Vec3d::new(0.0, 0.0, 0.0));
    camera.set_defocus_angle(0.0);

    let mut world = HittableVec::new();
    let perlin_texture: Arc<Box<dyn Texture>> = Arc::new(Box::new(
        PerlinTexture::new(4.0)
    ));
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

    let light = Material::Light(Light::from_color(Vec3d::new(4.0, 4.0, 4.0)));
    world.add(
        Arc::new(Box::new(Quad::new(
            Vec3d::new(3.0, 1.0, -2.0),
            Vec3d::new(2.0, 0.0, 0.0),
            Vec3d::new(0.0, 2.0, 0.0),
            light.clone(),
        )))
    );
    world.add(
        Arc::new(Box::new(
            Sphere::static_sphere(
                Vec3d::new(0.0, 7.0, 0.0),
                2.0,
                light.clone(),
            )
        ))
    );
    (camera, BVHNode::from_hittable_vec(Arc::new(world)))
}


pub fn cornell_box() -> (Camera, BVHNode) {
    let mut world = HittableVec::new();
    let red = Material::Lambertian(Lambertian::new(Vec3d::new(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new(Vec3d::new(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new(Vec3d::new(0.12, 0.45, 0.15)));
    let light = Material::Light(Light::from_color(Vec3d::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 555.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        green.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::zero(),
        Vec3d::new(0.0, 555.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        red.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(343.0, 554.0, 332.0),
        Vec3d::new(-130.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, -105.0),
        light.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::zero(),
        Vec3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        white.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(555.0, 555.0, 555.0),
        Vec3d::new(-555.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, -555.0),
        white.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(0.0, 0.0, 555.0),
        Vec3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 555.0, 0.0),
        white.clone(),
    ))));

    let box1 = bbox(
        Point3d::zero(),
        Point3d::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1: Arc<Box<dyn Hittable>> = Arc::new(Box::new(RotateY::new(
        Arc::new(Box::new(box1)),
        15.0,
    )));

    let box1: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Translate::new(
        box1,
        Vec3d::new(265.0, 0.0, 295.0),
    )));

    world.add(box1);

    let box2 = bbox(
        Point3d::zero(),
        Point3d::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    let box2: Arc<Box<dyn Hittable>> = Arc::new(Box::new(RotateY::new(
        Arc::new(Box::new(box2)),
        -18.0,
    )));

    let box2: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Translate::new(
        box2,
        Vec3d::new(130.0, 0.0, 65.0),
    )));

    world.add(box2);

    let mut camera = Camera::new();

    camera.set_aspect_ratio(1.0);
    camera.set_resolution_width(600);
    camera.set_samples_per_pixel(200);
    camera.set_depth(50);
    camera.set_background_color(Color::zero());
    camera.set_v_fov(40.0);
    camera.set_look_from(Point3d::new(278.0, 278.0, -800.0));
    camera.set_look_at(Point3d::new(278.0, 278.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    (camera, BVHNode::from_hittable_vec(Arc::new(world)))
}

pub fn cornell_smoke() -> (Camera, BVHNode) {
    let mut world = HittableVec::new();

    let red = Material::Lambertian(Lambertian::new(Vec3d::new(0.65, 0.05, 0.05)));
    let white = Material::Lambertian(Lambertian::new(Vec3d::new(0.73, 0.73, 0.73)));
    let green = Material::Lambertian(Lambertian::new(Vec3d::new(0.12, 0.45, 0.15)));
    let light = Material::Light(Light::from_color(Vec3d::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 555.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        green.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::zero(),
        Vec3d::new(0.0, 555.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        red.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(113.0, 554.0, 127.0),
        Vec3d::new(330.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, 305.0),
        light.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(0.0, 555.0, 0.0),
        Vec3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        white.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::zero(),
        Vec3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, 555.0),
        white.clone(),
    ))));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(0.0, 0.0, 555.0),
        Vec3d::new(555.0, 0.0, 0.0),
        Vec3d::new(0.0, 555.0, 0.0),
        white.clone(),
    ))));

    let box1 = bbox(
        Point3d::zero(),
        Point3d::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1: Arc<Box<dyn Hittable>> = Arc::new(Box::new(RotateY::new(
        Arc::new(Box::new(box1)),
        15.0,
    )));

    let box1: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Translate::new(
        box1,
        Vec3d::new(265.0, 0.0, 295.0),
    )));

    let box2 = bbox(
        Point3d::zero(),
        Point3d::new(165.0, 165.0, 165.0),
        white.clone(),
    );

    let box2: Arc<Box<dyn Hittable>> = Arc::new(Box::new(RotateY::new(
        Arc::new(Box::new(box2)),
        -18.0,
    )));

    let box2: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Translate::new(
        box2,
        Vec3d::new(130.0, 0.0, 65.0),
    )));

    world.add(
        Arc::new(Box::new(Medium::from_color(
            box1,
            0.01,
            Color::zero(),
        )))
    );
    world.add(
        Arc::new(Box::new(Medium::from_color(
            box2,
            0.01,
            Color::new(1.0, 1.0, 1.0),
        )))
    );


    let mut camera = Camera::new();

    camera.set_aspect_ratio(1.0);
    camera.set_resolution_width(600);
    camera.set_samples_per_pixel(200);
    camera.set_depth(50);
    camera.set_background_color(Color::zero());
    camera.set_v_fov(40.0);
    camera.set_look_from(Point3d::new(278.0, 278.0, -800.0));
    camera.set_look_at(Point3d::new(278.0, 278.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);

    (camera, BVHNode::from_hittable_vec(Arc::new(world)))
}

pub fn final_scene() -> (Camera, BVHNode) {
    let mut boxes1 = HittableVec::new();

    let ground = Material::Lambertian(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;
            let box_ = bbox(
                Point3d::new(x0, y0, z0),
                Point3d::new(x1, y1, z1),
                ground.clone(),
            );
            boxes1.add(Arc::new(Box::new(box_)))
        }
    }
    let mut world = HittableVec::new();
    world.add(Arc::new(Box::new(BVHNode::from_hittable_vec(Arc::new(boxes1)))));

    let light = Material::Light(Light::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Box::new(Quad::new(
        Point3d::new(123.0, 554.0, 147.0),
        Vec3d::new(300.0, 0.0, 0.0),
        Vec3d::new(0.0, 0.0, 265.0),
        light.clone(),
    ))));

    let center1 = Point3d::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3d::new(30.0, 0.0, 0.0);

    let sphere_material = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Box::new(Sphere::moving_sphere(center1, center2, 50.0, sphere_material.clone()))));

    world.add(Arc::new(Box::new(Sphere::static_sphere(
        Point3d::new(260.0, 150.0, 45.0),
        50.0,
        Material::Dielectric(Dielectric::new(1.5)),
    ))));

    world.add(Arc::new(Box::new(Sphere::static_sphere(
        Point3d::new(0.0, 150.0, 145.0),
        50.0,
        Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    ))));

    let boundary: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Sphere::static_sphere(
        Point3d::new(360.0, 150.0, 145.0),
        70.0,
        Material::Dielectric(Dielectric::new(1.5)),
    )));

    world.add(boundary.clone());

    world.add(Arc::new(Box::new(Medium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ))));

    let boundary: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Sphere::static_sphere(
        Point3d::zero(),
        5000.0,
        Material::Dielectric(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Box::new(Medium::from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ))));

    let image_file = "./misc/earthmap.png".to_string();
    let earth_texture: Arc<Box<dyn Texture>> = Arc::new(Box::new(ImageTexture::new(&image_file)));
    let emat = Material::Lambertian(Lambertian::from_texture(earth_texture));
    world.add(Arc::new(Box::new(Sphere::static_sphere(
        Point3d::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    ))));

    let pertext = PerlinTexture::new(0.2);
    world.add(Arc::new(Box::new(Sphere::static_sphere(
        Point3d::new(220.0, 280.0, 300.0),
        80.0,
        Material::Lambertian(Lambertian::from_texture(Arc::new(Box::new(pertext)))),
    ))));

    let mut boxes2 = HittableVec::new();
    let white = Material::Lambertian(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Box::new(Sphere::static_sphere(
            Vec3d::gen_range(0.0, 165.0),
            10.0,
            white.clone(),
        ))));
    }

    world.add(Arc::new(Box::new(Translate::new(
        Arc::new(Box::new(RotateY::new(
            Arc::new(Box::new(BVHNode::from_hittable_vec(Arc::new(boxes2)))),
            15.0,
        ))),
        Vec3d::new(-100.0, 270.0, 395.0),
    ))));

    let mut camera = Camera::new();
    camera.set_aspect_ratio(1.0);
    camera.set_resolution_width(1024);
    camera.set_samples_per_pixel(10000);
    camera.set_depth(50);
    camera.set_background_color(Color::zero());

    camera.set_v_fov(40.0);
    camera.set_look_from(Point3d::new(478.0, 278.0, -600.0));
    camera.set_look_at(Point3d::new(278.0, 278.0, 0.0));
    camera.set_v_up(Vec3d::new(0.0, 1.0, 0.0));
    camera.set_defocus_angle(0.0);
    (camera, BVHNode::from_hittable_vec(Arc::new(world)))
}