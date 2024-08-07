use crate::vec3d::Vec3d;

pub struct Camera {
    pub center: Vec3d,
    focal_length: f64,
    aspect_ratio: f64,

    pub resolution: (i32, i32),
    pub viewport_dims: (f64, f64),

    viewport_u: Vec3d,
    viewport_v: Vec3d,
}


impl Camera {
    pub fn new(
        center: Vec3d,
        focal_length: f64,
        aspect_ratio: f64,
        image_width: i32,
        viewport_height: f64,
    ) -> Self {
        let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3d::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3d::new(0.0, -viewport_height, 0.0);

        Self {
            center,
            focal_length,
            aspect_ratio,
            resolution: (image_width, image_height),
            viewport_dims: (viewport_width, viewport_height),
            viewport_u,
            viewport_v,
        }
    }

    pub fn resolution_width(&self) -> i32 {
        self.resolution.0
    }

    pub fn resolution_height(&self) -> i32 {
        self.resolution.1
    }

    pub fn viewport_width(&self) -> f64 {
        self.viewport_dims.0
    }

    pub fn viewport_height(&self) -> f64 {
        self.viewport_dims.1
    }

    pub fn pixel_delta_u(&self) -> Vec3d {
        // The pixel delta in the u (x) direction.
        self.viewport_u / self.resolution.0 as f64
    }

    pub fn pixel_delta_v(&self) -> Vec3d {
        // The pixel delta in the v (y) direction.
        self.viewport_v / self.resolution.1 as f64
    }

    pub fn viewport_upper_left(&self) -> Vec3d {
        self.center - self.viewport_u / 2.0 - self.viewport_v / 2.0 - Vec3d::new(0.0, 0.0, self.focal_length)
    }

    pub fn pixel_upper_left(&self) -> Vec3d {
        self.viewport_upper_left() + (self.pixel_delta_u() + self.pixel_delta_v()) * 0.5
    }

    /// Returns the center of the pixel at the given width and height coordinate.
    /// # Arguments
    /// * `w` - The width coordinate of the pixel.
    /// * `h` - The height coordinate of the pixel.
    pub fn pixel_center(&self, w: i32, h: i32) -> Vec3d {
        self.pixel_upper_left() + self.pixel_delta_u() * w as f64 + self.pixel_delta_v() * h as f64
    }
}

