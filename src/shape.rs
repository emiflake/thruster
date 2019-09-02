/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   shape.rs                                           :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:17:32 by nmartins       #+#    #+#                */
/*   Updated: 2019/08/05 16:40:25 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::algebra::{Vec2, Vec3, Vertex};
use crate::material::MatTex;
use crate::scene::Scene;

use rand::prelude::*;

pub type Shape<'a> = Box<dyn SceneObject + 'a + Sync>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub level: i32,

    pub inv_dir: Vec3,
    pub sign: Vec3,
}

impl Ray {
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub normal: Vec3,
    pub origin: Vec3,
    pub text_pos: Vec2,
}

pub struct BoundingBox {
    pub min_vector: Vec3,
    pub max_vector: Vec3,
}

impl BoundingBox {
    fn bounds(&self, sign: f64) -> Vec3 {
        if sign == 1.0 {
            self.min_vector
        } else {
            self.max_vector
        }
    }

    pub fn centre(&self) -> Vec3 {
        (self.min_vector + self.max_vector) / 2.0
    }

    pub fn intersects_with(&self, ray: &Ray) -> bool {
        let mut tmin = (self.bounds(ray.sign.x).x - ray.origin.x) * ray.inv_dir.x;
        let mut tmax = (self.bounds(1.0 - ray.sign.x).x - ray.origin.x) * ray.inv_dir.x;
        let tymin = (self.bounds(ray.sign.y).y - ray.origin.y) * ray.inv_dir.y;
        let tymax = (self.bounds(1.0 - ray.sign.y).y - ray.origin.y) * ray.inv_dir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds(ray.sign.z).z - ray.origin.z) * ray.inv_dir.z;
        let tzmax = (self.bounds(1.0 - ray.sign.z).z - ray.origin.z) * ray.inv_dir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        let mut t = tmin;

        if t < 0.0 {
            t = tmax;
            if t < 0.0 {
                return false;
            }
        }

        true
    }
}

use crate::material::Material;
pub trait SceneObject {
    fn mat(&self) -> &Material;
    /* Whether or not object intersects with the ray */
    fn do_intersect(&self, ray: &Ray) -> Option<Intersection>;

    fn bounding_box(&self) -> BoundingBox;
}

impl Ray {
    pub fn cast<'a>(&self, scene: &'a Scene) -> Vec<(Intersection, &'a Shape<'a>)> {
        scene.bvh.intersect(self)
    }

    pub fn color_function<'a>(
        &self,
        intersections: Vec<(Intersection, &Shape<'a>)>,
        scene: &Scene,
    ) -> Option<Vec3> {
        let mut rng = rand::thread_rng();
        let mut closest;
        closest = intersections.first()?;
        for intersection in intersections.iter() {
            if closest.0.t > intersection.0.t {
                closest = intersection;
            }
        }
        let inter = closest.0;
        let mat = closest.1.mat();
        let orig_color = match &mat.texture {
            MatTex::Color(col) => *col,
            MatTex::Texture { handle, scaling } => {
                let text = scene.texture_map.get_image_by_handle(*handle).unwrap();

                let pixel = text.get_pixel(
                    (inter.text_pos.x * f64::from(text.width()) / scaling.x) as u32 % text.width(),
                    (inter.text_pos.y * f64::from(text.height()) / scaling.y) as u32
                        % text.height(),
                );
                Vec3::from_rgb(*pixel)
            }
        };
        let mut diff_color = Vec3::ORIGIN;
        for light in scene.lights.iter() {
            diff_color = diff_color
                + orig_color * (light.color() / 255.0) * light.luminosity_at(scene, &inter);
        }
        let n_dot_d = inter.normal.dot(&self.direction);
        let refr_color = {
            if self.level <= 0 || !mat.transparency.is_transparent() || !scene.config.reflections {
                Vec3::ORIGIN
            } else {
                let ior = mat.transparency.index_of_refraction;
                let eta = 2.0 - ior;
                let o = self.direction * eta - inter.normal * (-n_dot_d + eta * n_dot_d);
                let ray = Ray::new(inter.origin - inter.normal * 0.01, o, self.level - 1);
                match ray.color_function(ray.cast(scene), scene) {
                    Some(color) => color,
                    _ => {
                        if scene.config.skybox {
                            scene
                                .skybox
                                .calc_color(scene, ray.direction)
                                .unwrap_or(Vec3::ORIGIN)
                        } else {
                            Vec3::ORIGIN
                        }
                    }
                }
            }
        };
        let refl_color = {
            if self.level == 0 || !mat.reflectivity.is_reflective() || !scene.config.reflections {
                Vec3::ORIGIN
            } else {
                if scene.config.distributed_tracing {
                    let mut col = Vec3::ORIGIN;
                    let blurriness = mat.reflectivity.blurriness;
                    let spp = if blurriness == 0.0 {
                        1
                    } else {
                        scene.config.reflection_spp
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
                } else {
                    let reflection_dir = self.direction - (n_dot_d * 2.0) * inter.normal;
                    let ray = Ray::new(
                        inter.origin + inter.normal * 0.01,
                        reflection_dir,
                        self.level - 1,
                    );
                    match ray.color_function(ray.cast(scene), scene) {
                        Some(color) => color,
                        _ => {
                            if scene.config.skybox {
                                scene
                                    .skybox
                                    .calc_color(scene, ray.direction)
                                    .unwrap_or(Vec3::ORIGIN)
                            } else {
                                Vec3::ORIGIN
                            }
                        }
                    }
                }
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

#[derive(Clone)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,

    pub material: Material,
}

impl SceneObject for Sphere {
    fn mat(&self) -> &Material {
        &self.material
    }

    fn do_intersect(&self, ray: &Ray) -> Option<Intersection> {
        let local_ray = self.origin - ray.origin;
        let tca = local_ray.dot(&ray.direction);
        if tca < 0.0 {
            return None;
        }
        let d2 = local_ray.length2() - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        let t = if t0 < 0.0 {
            t1
        } else if t1 < 0.0 {
            t0
        } else {
            t0.min(t1)
        };
        if t <= 0.0 {
            return None;
        }
        let p = ray.origin + (ray.direction * t);
        let normal = (p - self.origin).normalized();
        let text_pos = Vec2::new(
            0.5 + normal.z.atan2(normal.x) / std::f64::consts::PI / 2.0,
            0.5 - normal.y.asin() / std::f64::consts::PI,
        );
        Some(Intersection {
            t,
            origin: p,
            normal,
            text_pos,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min_vector: self.origin - Vec3::new(1.0, 1.0, 1.0) * self.radius,
            max_vector: self.origin + Vec3::new(1.0, 1.0, 1.0) * self.radius,
        }
    }
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,

    pub material: Material,
}

impl SceneObject for Plane {
    fn mat(&self) -> &Material {
        &self.material
    }

    fn do_intersect(&self, ray: &Ray) -> Option<Intersection> {
        let t = (self.origin - ray.origin).dot(&self.normal) / (self.normal.dot(&ray.direction));
        if t < 0.001 {
            None
        } else {
            let p = ray.origin + (ray.direction * t);
            let text_pos = Vec2::new(p.x, p.z);
            Some(Intersection {
                origin: p,
                t,
                normal: self.normal,
                text_pos,
            })
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        let far = Vec3::new(1000.0, 1000.0, 1000.0);

        BoundingBox {
            min_vector: self.origin - far + self.normal * far,
            max_vector: self.origin + far - self.normal * far,
        }
    }
}

#[allow(dead_code)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,

    pub material: Material,
}

impl SceneObject for Triangle {
    fn mat(&self) -> &Material {
        &self.material
    }

    fn do_intersect(&self, ray: &Ray) -> Option<Intersection> {
        let ab = self.b.origin - self.a.origin;
        let ac = self.c.origin - self.a.origin;

        let pvec = ray.direction.cross_product(&ac);
        let det = ab.dot(&pvec);

        if det < std::f64::EPSILON {
            return None;
        }
        if det.abs() < std::f64::EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.a.origin;
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let qvec = tvec.cross_product(&ab);
        let v = ray.direction.dot(&qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = ac.dot(&qvec) * inv_det;
        if t < 0.0 {
            return None;
        }
        let p = ray.origin + ray.direction * t;

        Some(Intersection {
            origin: p,
            t,
            normal: self.a.normal.normalized() * (1.0 - u - v)
                + self.b.normal.normalized() * u
                + self.c.normal.normalized() * v,
            text_pos: Vec2::new(
                self.a.uv.x * (1.0 - u - v) + self.b.uv.x * u + self.c.uv.x * v,
                self.a.uv.y * (1.0 - u - v) + self.b.uv.y * u + self.c.uv.y * v,
            ),
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min_vector: self.a.origin.min(self.b.origin).min(self.c.origin),
            max_vector: self.a.origin.max(self.b.origin).max(self.c.origin),
        }
    }
}
