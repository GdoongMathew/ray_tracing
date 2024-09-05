use crate::vec3d::{Vec3d, Color, Point3d, cross};
use crate::object::Hittable;
use crate::ray::{Ray, Interval};
use rand::Rng;
use crate::object::material::Scatterable;
use indicatif::ProgressBar;

use std::thread;
use rayon;
use std::sync::mpsc;

#[derive(Copy, Clone)]
pub struct Camera {
    center: Point3d,
    aspect_ratio: f64,

    resolution: (i32, i32),
    viewport_dims: (f64, f64),

    viewport_u: Vec3d,
    viewport_v: Vec3d,

    samples_per_pixel: i32,
    samples_scale: f64,

    max_depth: i32,

    v_fov: f64, // Vertical field of view in degrees.

    look_from: Point3d,   // Point camera is looking from
    look_at: Vec3d,     // Point camera is looking at
    v_up: Vec3d,        // Camera-relative up vector

    defocus_angle: f64,
    defocus_radius: f64,
    focus_dist: f64,

    background_color: Color,

}


impl Camera {
    pub fn new() -> Self {
        let center = Point3d::zero();
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 1080;
        let v_fov: f64 = 90.0;

        let look_from = Point3d::new(0.0, 0.0, 0.0);
        let look_at = Vec3d::new(0.0, 0.0, -1.0);
        let v_up = Vec3d::new(0.0, 1.0, 0.0);
        let focal_length = (look_from - look_at).length();

        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;

        let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3d::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3d::new(0.0, -viewport_height, 0.0);

        Self {
            center,
            aspect_ratio,
            resolution: (image_width, image_height),
            viewport_dims: (viewport_width, viewport_height),
            viewport_u,
            viewport_v,
            samples_per_pixel: 1,
            samples_scale: 1.0,
            max_depth: 10,
            v_fov,
            look_from,
            look_at,
            v_up,
            defocus_angle: 0.0,
            defocus_radius: 0.0,
            focus_dist: 10.0,
            background_color: Color::zero(),
        }
    }

    fn initialize(&mut self) -> () {
        self.update_resolution_height();
        self.set_center(self.look_from);

        let h = (self.theta() / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.resolution_width() as f64 / self.resolution_height() as f64);
        self.viewport_dims = (viewport_width, viewport_height);

        self.viewport_u = self.u() * self.viewport_width();
        self.viewport_v = -self.v() * self.viewport_height();

        self.defocus_radius = (self.defocus_angle / 2.0).to_radians().tan() * self.focus_dist;
    }

    fn theta(&self) -> f64 { self.v_fov.to_radians() }

    fn w(&self) -> Vec3d { (self.look_from - self.look_at).unit_vector() }

    fn u(&self) -> Vec3d { cross(&self.v_up, &self.w()).unit_vector() }

    fn v(&self) -> Vec3d { cross(&self.w(), &self.u()) }

    pub fn set_look_from(&mut self, look_from: Vec3d) -> () { self.look_from = look_from; }
    pub fn set_look_at(&mut self, look_at: Vec3d) -> () { self.look_at = look_at; }
    pub fn set_v_up(&mut self, v_up: Vec3d) -> () { self.v_up = v_up; }

    pub fn focal_length(&self) -> f64 { (self.look_from - self.look_at).length() }

    fn set_center(&mut self, center: Vec3d) -> () { self.center = center; }

    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: i32) -> () {
        self.samples_per_pixel = samples_per_pixel;
        self.samples_scale = 1.0 / (samples_per_pixel as f64);
    }

    pub fn set_v_fov(&mut self, v_fov: f64) -> () { self.v_fov = v_fov; }

    pub fn set_depth(&mut self, max_depth: i32) -> () { self.max_depth = max_depth; }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> () { self.aspect_ratio = aspect_ratio; }

    pub fn set_resolution_width(&mut self, width: i32) -> () { self.resolution.0 = width; }

    fn update_resolution_height(&mut self) -> () {
        let height = (self.resolution_width() as f64 / self.aspect_ratio) as i32;
        self.resolution.1 = height.max(1);
    }

    pub fn set_defocus_angle(&mut self, angle: f64) -> () { self.defocus_angle = angle; }

    pub fn set_focus_dist(&mut self, focus_dist: f64) -> () { self.focus_dist = focus_dist; }

    pub fn set_background_color(&mut self, color: Color) -> () { self.background_color = color; }

    fn defocus_disk_u(&self) -> Vec3d { self.u() * self.defocus_radius }

    fn defocus_disk_v(&self) -> Vec3d { self.v() * self.defocus_radius }

    pub fn resolution_width(&self) -> i32 { self.resolution.0 }

    pub fn resolution_height(&self) -> i32 { self.resolution.1 }

    pub fn viewport_width(&self) -> f64 { self.viewport_dims.0 }

    pub fn viewport_height(&self) -> f64 { self.viewport_dims.1 }

    pub fn pixel_delta_u(&self) -> Vec3d {
        // The pixel delta in the u (x) direction.
        self.viewport_u / self.resolution_width() as f64
    }

    pub fn pixel_delta_v(&self) -> Vec3d {
        // The pixel delta in the v (y) direction.
        self.viewport_v / self.resolution_height() as f64
    }

    pub fn viewport_upper_left(&self) -> Point3d {
        self.center - self.w() * self.focus_dist - self.viewport_u / 2.0 - self.viewport_v / 2.0
    }

    pub fn pixel_upper_left(&self) -> Point3d {
        self.viewport_upper_left() + (self.pixel_delta_u() + self.pixel_delta_v()) * 0.5
    }

    /// Returns the center of the pixel at the given width and height coordinate.
    /// # Arguments
    /// * `w` - The width coordinate of the pixel.
    /// * `h` - The height coordinate of the pixel.
    pub fn pixel_coords(&self, w: f64, h: f64) -> Point3d {
        self.pixel_upper_left() + self.pixel_delta_u() * w + self.pixel_delta_v() * h
    }

    fn ray_color<H: Hittable>(ray: &Ray, world: &H, depth: i32, background: &Color) -> Color {
        if depth <= 0 { return Color::zero(); }

        if let Some(hit_record) = world.hit(ray, &Interval { min: 0.0001, max: f64::INFINITY }) {
            let emitted = hit_record.material.emitted(hit_record.u, hit_record.v, &hit_record.point);

            if let Some((scattered_ray, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
                let color = attenuation * Self::ray_color(
                    &scattered_ray, world, depth - 1, background
                );
                return color + emitted;
            }
            emitted
        } else {
            // hits nothing.
            *background
        }
    }

    /// Random sample a ray through the pixel at the given width and height coordinate.
    /// # Arguments
    /// * `i` - The width coordinate of the pixel.
    /// * `j` - The height coordinate of the pixel.
    fn sample_ray(&self, i: i32, j: i32) -> Ray {
        let mut rng = rand::thread_rng();

        let (offset_i, offset_j) = rng.random::<(f64, f64)>();

        let pixel_sample = self.pixel_coords(
            i as f64 + offset_i,
            j as f64 + offset_j,
        );

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, direction, rng.random::<f64>())
    }

    fn defocus_disk_sample(&self) -> Vec3d {
        let p = Vec3d::random_in_unit_disk();
        self.center + self.defocus_disk_u() * p.x() + self.defocus_disk_v() * p.y()
    }

    pub fn render<H: Hittable>(&mut self, world: &'static H) -> Vec<Vec3d> {
        self.initialize();

        let mut image = vec![
            Vec3d::new(0.0, 0.0, 0.0);
            (self.resolution_width() * self.resolution_height()) as usize
        ];

        let bar = ProgressBar::new(
            self.resolution_height() as u64 * self.resolution_width() as u64
        );

        // Multi threading computation
        let available_threads = thread::available_parallelism().unwrap().get();
        let num_threads = (available_threads as f32 * 0.75) as usize;

        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
        let (tx, rx) = mpsc::channel();

        rayon::scope(|s| {
            for h in 0..self.resolution_height() {
                for w in 0..self.resolution_width() {
                    let tx_clone = tx.clone();
                    let camera = self.clone();

                    thread_pool.spawn(move || {
                        let mut color = Vec3d::zero();
                        for _ in 0..camera.samples_per_pixel {
                            let ray = camera.sample_ray(w, h);
                            color += Self::ray_color(&ray, world, camera.max_depth, &camera.background_color);
                        }
                        tx_clone.send((w, h, color * camera.samples_scale)).unwrap();
                    })
                }
            }
        });


        for _ in 0..(self.resolution_height() * self.resolution_width()) {
            let (w, h, color) = rx.recv().unwrap();
            image[(h * self.resolution_width() + w) as usize] = color;
            bar.inc(1);
        }
        bar.finish_and_clear();
        image
    }
}

