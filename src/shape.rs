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

pub type Shape<'a> = Box<dyn Intersectable + 'a + Sync>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub normal: Vec3,
    pub origin: Vec3,
    pub text_pos: Vec2,
}

use crate::material::Material;
pub trait Intersectable {
    fn mat(&self) -> &Material;
    /* Whether or not object intersects with the ray */
    fn do_intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Ray {
    pub fn cast<'a>(&self, scene: &'a Scene) -> Vec<(Intersection, &'a Shape<'a>)> {
        let mut intersections: Vec<(Intersection, &Shape<'a>)> = Vec::new();

        for shape in scene.shapes.iter() {
            if let Some(intersection) = shape.do_intersect(self) {
                intersections.push((intersection, shape));
            }
        }

        intersections
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
            diff_color = diff_color + orig_color * light.luminosity_at(scene, &inter);
        }
        let n_dot_d = inter.normal.dot(&self.direction);
        let refr_color = {
            if self.level <= 0 || !mat.transparency.is_transparent() {
                Vec3::ORIGIN
            } else {
                let ior = mat.transparency.index_of_refraction;
                let eta = 2.0 - ior;
                let o = self.direction * eta - inter.normal * (-n_dot_d + eta * n_dot_d);
                let ray = Ray {
                    origin: inter.origin - inter.normal * 0.01,
                    direction: o,
                    level: self.level - 1,
                };
                match ray.color_function(ray.cast(scene), scene) {
                    Some(color) => color,
                    _ => scene
                        .skybox
                        .calc_color(scene, ray.direction)
                        .unwrap_or(Vec3::ORIGIN),
                }
            }
        };
        let refl_color = {
            if self.level == 0 || mat.c_reflection <= 0.0 {
                Vec3::ORIGIN
            } else {
                let mut col = Vec3::ORIGIN;
                let spp = 5;
                let blurriness = 1.0;

                for _ in 0..spp {
                    let reflection_dir = self.direction - (n_dot_d * 2.0) * inter.normal;
                    let ray = Ray {
                        origin: inter.origin + inter.normal * 0.01,
                        direction: reflection_dir.rotate(Vec3::new(
                            (rng.gen::<f64>() - 0.5) * blurriness,
                            (rng.gen::<f64>() - 0.5) * blurriness,
                            (rng.gen::<f64>() - 0.5) * blurriness,
                        )),
                        level: self.level - 1,
                    };
                    col = col
                        + match ray.color_function(ray.cast(scene), scene) {
                            Some(color) => color / f64::from(spp),
                            _ => {
                                (scene
                                    .skybox
                                    .calc_color(scene, ray.direction)
                                    .unwrap_or(Vec3::ORIGIN))
                                    / f64::from(spp)
                            }
                        };
                }
                col
            }
        };
        Some(
            orig_color.clamp_as_color() * mat.c_ambient
                + diff_color.clamp_as_color() * mat.c_diffuse
                + refl_color.clamp_as_color() * mat.c_reflection
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

impl Intersectable for Sphere {
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
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,

    pub material: Material,
}

impl Intersectable for Plane {
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
}

#[allow(dead_code)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,

    pub material: Material,
}

impl Intersectable for Triangle {
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
}
