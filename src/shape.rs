/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   shape.rs                                           :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:17:32 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/25 17:00:12 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

trait Clampable {
    fn clamp_to(self, min: Self, max: Self) -> Self;
}

impl Clampable for f64 {
    fn clamp_to(self, min: f64, max: f64) -> f64 {
        if self > max {
            return max;
        }
        if self < min {
            return min;
        }
        self
    }
}

impl Vec3 {
    pub const ORIGIN: Self = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn length2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn normalized(&self) -> Self {
        let mag = self.length();

        /* Make a copy and divide it by the magnitude*/
        *self / mag
    }

    pub fn distance2(&self, other: &Vec3) -> f64 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        self.distance2(other).sqrt()
    }

    pub fn clamp_as_color(&self) -> Self {
        self.clamp_to(Vec3::ORIGIN, Vec3::new(255.0, 255.0, 255.0))
    }

    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Clampable for Vec3 {
    fn clamp_to(self, min: Vec3, max: Vec3) -> Vec3 {
        Vec3::new(
            self.x.clamp_to(min.x, max.x),
            self.y.clamp_to(min.y, max.y),
            self.z.clamp_to(min.z, max.z),
        )
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub origin: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(origin: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self { origin, normal, uv }
    }

    pub fn from_parsed(vertex: &crate::parser::Vertex3) -> Self {
        Self {
            origin: Vec3::new(vertex.pos.x, vertex.pos.y, vertex.pos.z),
            normal: Vec3::new(vertex.normal.x, vertex.normal.y, vertex.normal.z),
            uv: Vec2::new(vertex.uv.x, vertex.uv.y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    /*
    	** Where the ray is cast from
    	*/
    pub origin: Vec3,

    /*
    	** (NORMALIZED)
    	** In what direction the ray is to be cast
    	*/
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
    pub fn cast<'a>(
        &self,
        scene: &'a crate::thruster::Thruster,
    ) -> Vec<(Intersection, &'a Box<dyn Intersectable + 'a>)> {
        let mut intersections: Vec<(Intersection, &Box<dyn Intersectable + 'a>)> = Vec::new();

        for shape in scene.shapes.iter() {
            if let Some(intersection) = shape.do_intersect(self) {
                intersections.push((intersection, shape));
            }
        }

        intersections
    }

    pub fn color_function<'a>(
        &self,
        intersections: Vec<(Intersection, &Box<dyn Intersectable + 'a>)>,
        scene: &crate::thruster::Thruster,
    ) -> Option<Vec3> {
        let mut closest;
        closest = intersections.first()?;
        for intersection in intersections.iter() {
            if closest.0.t > intersection.0.t {
                closest = intersection;
            }
        }
        use crate::material::MatTex;
        use image::Pixel;
        let mat = closest.1.mat();
        let inter = closest.0;
        let orig_color = match &mat.texture {
            MatTex::Color(col) => *col,
            MatTex::Texture { handle, scaling } => {
				let text = scene.texture_map.get_image_by_handle(*handle).unwrap();

                let channels = text
                    .get_pixel(
                        (inter.text_pos.x * f64::from(text.width()) / scaling.x) as u32
                            % text.width(),
                        (inter.text_pos.y * f64::from(text.height()) / scaling.y) as u32
                            % text.height(),
                    )
                    .channels();
                Vec3::new(
                    f64::from(channels[0]),
                    f64::from(channels[1]),
                    f64::from(channels[2]),
                )
            }
        };
        let mut diff_color = Vec3::ORIGIN;
        for light in scene.lights.iter() {
            diff_color = diff_color + orig_color * light.luminosity_at(scene, &closest.0);
        }
        let refl_color = {
            if mat.c_reflection <= 0.0 {
                Vec3::ORIGIN
            } else {
                let reflection_dir = self.direction
                    - (self.direction.dot(&closest.0.normal) * 2.0) * closest.0.normal;
                let ray = Ray {
                    origin: closest.0.origin,
                    direction: reflection_dir,
                    level: self.level - 1,
                };
                ray.color_function(ray.cast(scene), scene)
                    .or(Some(Vec3::ORIGIN))?
            }
        };
        Some(
            orig_color.clamp_as_color() * mat.c_ambient
                + diff_color.clamp_as_color() * mat.c_diffuse
                + refl_color.clamp_as_color() * mat.c_reflection,
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
        let t1 = tca - thc;
        let t = t0.max(t1);
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
            normal: self.a.normal * u + self.b.normal * v + self.c.normal * (1.0 - u - v),
            text_pos: Vec2::new(0.0, 0.0),
        })
    }
}
