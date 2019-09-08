use crate::algebra::prelude::*;
use crate::lightsource::Light;
use crate::material::MatTex;
use crate::scene::RenderData;
use crate::shape::{SceneObject, Shape};
use rand::prelude::*;

/// A Ray that is to be casted, should be created using the `new` function
#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    /// The position the ray starts from
    pub origin: Vec3,

    /// The direction the ray is cast towards
    pub direction: Vec3,

    /// The maximum recursion level
    pub level: i32,

    /// *Precomputed* value for some uses.
    pub inv_dir: Vec3,
    /// *Precomputed* value for some uses.
    pub sign: Vec3,
}

impl Ray {
    /// The preferred way to create a Ray. This function already precomputes `inv_dir` and `sign`
    pub fn new(origin: Vec3, direction: Vec3, level: i32) -> Self {
        Self {
            origin,
            direction,
            level,
            inv_dir: Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z),
            sign: Vec3::new(
                if direction.x > 0.0 { 1.0 } else { 0.0 },
                if direction.y > 0.0 { 1.0 } else { 0.0 },
                if direction.z > 0.0 { 1.0 } else { 0.0 },
            ),
        }
    }

    /// Use the [BVHTree](../bvh/struct.BVHTree.html) to find the nearest intersection
    pub fn cast<'a>(&self, scene: &'a RenderData) -> Option<(Intersection, &'a Shape)> {
        scene.bvh.intersect(self)
    }

    /// The color function of a Ray, allows it to generate coloring for a Ray trace.
    ///
    /// **TODO:** Allow simplify this function in some way, possibly by abstracting, this is
    /// complicated due to the complex nature of the equation.
    pub fn color_function(
        &self,
        closest: Option<(Intersection, &Shape)>,
        scene: &RenderData,
    ) -> Option<Vec3> {
        let mut rng = rand::thread_rng();
        let (inter, shape) = closest?;
        let mat = shape.mat();
        let orig_color = match &mat.texture {
            MatTex::Color(col) => *col,
            MatTex::Texture { handle, scaling } => {
                if scene.config.textures {
                    let text = scene
                        .texture_map
                        .get_image_by_handle(handle.clone())
                        .unwrap();

                    let pixel = text.get_pixel(
                        (inter.text_pos.x * f64::from(text.width()) / scaling.x) as u32
                            % text.width(),
                        (inter.text_pos.y * f64::from(text.height()) / scaling.y) as u32
                            % text.height(),
                    );
                    Vec3::from_rgb(*pixel)
                } else {
                    Vec3::new(127.0, 127.0, 127.0)
                }
            }
        };
        let mut diff_color = Vec3::ORIGIN;
        for light in scene.lights.iter() {
            diff_color = diff_color
                + orig_color * (light.color() / 255.0) * light.luminosity_at(scene, &inter);
        }
        let n_dot_d = comb::dot(&inter.normal, &self.direction);
        let refr_color = {
            if self.level <= 0 || !mat.transparency.is_transparent() || !scene.config.refractions {
                Vec3::ORIGIN
            } else {
                let mut col = Vec3::ORIGIN;
                let (spp, blurriness) = if scene.config.distributed_tracing {
                    let blurriness = mat.reflectivity.blurriness;
                    (
                        if blurriness == 0.0 {
                            1
                        } else {
                            scene.config.reflection_spp
                        },
                        blurriness,
                    )
                } else {
                    (1, 0.0)
                };

                let ior = mat.transparency.index_of_refraction;
                let eta = 2.0 - ior;
                let o = self.direction * eta - inter.normal * (-n_dot_d + eta * n_dot_d);
                for _ in 0..spp {
                    let ray = Ray::new(
                        inter.origin - inter.normal * 0.01,
                        o.rotate(Vec3::new(
                            (rng.gen::<f64>() - 0.5) * blurriness,
                            (rng.gen::<f64>() - 0.5) * blurriness,
                            (rng.gen::<f64>() - 0.5) * blurriness,
                        )),
                        self.level - 1,
                    );
                    match ray.color_function(ray.cast(scene), scene) {
                        Some(color) => col = col + color,
                        _ => {
                            if scene.config.skybox {
                                col = col
                                    + scene
                                        .skybox
                                        .calc_color(scene, ray.direction)
                                        .unwrap_or(Vec3::ORIGIN)
                            }
                        }
                    }
                }
                col / f64::from(spp)
            }
        };
        let refl_color = {
            if self.level == 0 || !mat.reflectivity.is_reflective() || !scene.config.reflections {
                Vec3::ORIGIN
            } else {
                let mut col = Vec3::ORIGIN;
                let (spp, blurriness) = if scene.config.distributed_tracing {
                    let blurriness = mat.reflectivity.blurriness;
                    (
                        if blurriness == 0.0 {
                            1
                        } else {
                            scene.config.reflection_spp
                        },
                        blurriness,
                    )
                } else {
                    (1, 0.0)
                };

                for _ in 0..spp {
                    let reflection_dir = self.direction - (n_dot_d * 2.0) * inter.normal;
                    let ray = Ray::new(
                        inter.origin + inter.normal * 0.01,
                        reflection_dir.rotate(Vec3::new(
                            (rng.gen::<f64>() - 0.5) * blurriness,
                            (rng.gen::<f64>() - 0.5) * blurriness,
                            (rng.gen::<f64>() - 0.5) * blurriness,
                        )),
                        self.level - 1,
                    );
                    col = col
                        + match ray.color_function(ray.cast(scene), scene) {
                            Some(color) => color / f64::from(spp),
                            _ => {
                                if scene.config.skybox {
                                    (scene
                                        .skybox
                                        .calc_color(scene, ray.direction)
                                        .unwrap_or(Vec3::ORIGIN))
                                        / f64::from(spp)
                                } else {
                                    Vec3::ORIGIN
                                }
                            }
                        };
                }
                col
            }
        };
        Some(
            orig_color.clamp_as_color() * mat.c_ambient
                + diff_color.clamp_as_color() * mat.c_diffuse
                + refl_color.clamp_as_color() * mat.reflectivity.amount
                + refr_color.clamp_as_color() * mat.transparency.amount,
        )
    }
}
