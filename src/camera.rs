use crate::algebra::prelude::*;
use crate::scene::RenderData;
use rand::prelude::*;

use serde_derive::{Deserialize, Serialize};

/// A camera is an entity that can cast [ray](../shape/struct.Ray.html)s.
pub trait Camera {
    /// Project a Ray into the scene
    fn project_rays(
        &self,
        screen_pos: (f64, f64),
        screen_dim: (f64, f64),
        scene: &RenderData,
    ) -> Vec<Ray>;
}

/// A very simple Perspective Camera which will cast rays in a natural way.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PerspectiveCamera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub fov: f64,
}

impl PerspectiveCamera {
    #[allow(dead_code)]
    pub fn new(position: Vec3) -> Self {
        PerspectiveCamera {
            position,
            rotation: Vec3::ORIGIN,
            fov: 60.0,
        }
    }

    /// Move the Camera by a Vector
    #[allow(dead_code)]
    pub fn translate(&mut self, v3: Vec3) {
        self.position.x += self.rotation.y.cos() * v3.x - self.rotation.y.sin() * v3.z;
        self.position.y += v3.y;
        self.position.z += -self.rotation.y.cos() * v3.z - self.rotation.y.sin() * v3.x;
    }

    pub fn rotate(&mut self, v3: Vec3) {
        self.rotation = self.rotation + v3;
    }
}

const DISTRIB_CAMERA: bool = false;

impl Camera for PerspectiveCamera {
    fn project_rays(
        &self,
        (sx, sy): (f64, f64),
        (w, h): (f64, f64),
        scene: &RenderData,
    ) -> Vec<Ray> {
        if DISTRIB_CAMERA && scene.config.distributed_tracing {
            let mut rng = rand::thread_rng();
            let aspect_ratio = w / h;
            let px = (2.0 * ((sx + 0.5) / w) - 1.0)
                * aspect_ratio
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
            let py = (1.0 - 2.0 * ((sy + 0.5) / h))
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();

            let direction = Vec3::new(px, py, 1.0).normalized().rotate(self.rotation);

            let blurriness = 0.0001;
            let spp = 1;

            let mut rays: Vec<Ray> = Vec::with_capacity(spp);
            for _ in 0..spp {
                let sample_direction = direction.rotate(Vec3::new(
                    (rng.gen::<f64>() - 0.5) * blurriness,
                    (rng.gen::<f64>() - 0.5) * blurriness,
                    (rng.gen::<f64>() - 0.5) * blurriness,
                ));
                rays.push(Ray::new(
                    self.position,
                    sample_direction,
                    scene.config.recursion_depth,
                ));
            }
            rays
        } else {
            let aspect_ratio = w / h;
            let px = (2.0 * ((sx + 0.5) / w) - 1.0)
                * aspect_ratio
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
            let py = (1.0 - 2.0 * ((sy + 0.5) / h))
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();

            let direction = Vec3::new(px, py, 1.0).normalized().rotate(self.rotation);

            vec![Ray::new(
                self.position,
                direction,
                scene.config.recursion_depth,
            )]
        }
    }
}
