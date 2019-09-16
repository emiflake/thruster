use crate::algebra::prelude::*;

pub struct CameraSample {
    pub film_pos: Point2,
    pub time: f64,
}

pub trait Camera {
    fn generate_ray(&self, sample: &CameraSample) -> Ray;
}

pub struct PerspectiveCamera {
    pub camera_to_world: Transform,
    pub raster_to_screen: Transform,
    pub shutter_time: (f64, f64),
    pub fov: f64,
    pub scaling: Vec3,
    pub screen_dim: Vec2,
    pub proj_dir_inv: Transform,
}

impl PerspectiveCamera {
    pub fn new(camera_to_world: Transform, fov: f64, screen_dimensions: Vec2) -> Self {
        let shutter_time = (0.0, 1.0);
        let aspect_ratio = screen_dimensions.x / screen_dimensions.y;
        let screen = if aspect_ratio > 1.0 {
            (-aspect_ratio, aspect_ratio, -1.0, 1.0)
        } else {
            (-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio)
        };
        let screen_to_raster = Transform::scaling(screen_dimensions.x, screen_dimensions.y, 1.0)
            * Transform::scaling(
                1.0 / (screen.1 - screen.0),
                1.0 / (screen.2 - screen.3),
                1.0,
            )
            * Transform::translation(&Vec3::new(-screen.0, -screen.3, 0.0));

        let raster_to_screen = screen_to_raster.inverse();
        let far = 1.0;
        let near = 1000.0;
        let proj_dir_inv = Transform::perspective(fov, far, near).inverse();
        let tan_fov = f64::tan(comb::to_radians(fov) / 2.0);
        let scaling = Vec3::new(tan_fov, tan_fov, 1.0);
        Self {
            proj_dir_inv,
            screen_dim: screen_dimensions,
            scaling,
            raster_to_screen,
            shutter_time,
            fov: comb::to_radians(fov),
            camera_to_world,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: &CameraSample) -> Ray {
        let px_pos = self.scaling
            * (Point3::new(
                sample.film_pos.x - self.screen_dim.x / 2.0,
                sample.film_pos.y - self.screen_dim.y / 2.0,
                0.0,
            )
            .apply_t(&self.proj_dir_inv)
            .apply_t(&self.raster_to_screen));
        let d = Vec3::new(px_pos.x, px_pos.y, px_pos.z).normalized();
        Ray::new(Point3::ORIGIN, d).apply_t(&self.camera_to_world)
    }
}
