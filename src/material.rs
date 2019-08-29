/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   material.rs                                        :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/20 19:44:22 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/27 15:06:24 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::algebra::{Vec2, Vec3};

use crate::texture_map::TextureHandle;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum MatTex {
    Color(Vec3),
    Texture {
        handle: TextureHandle,
        scaling: Vec2,
    },
}

impl MatTex {
    #[allow(dead_code)]
    pub fn from_color<'a>(x: f64, y: f64, z: f64) -> MatTex {
        MatTex::Color(Vec3 { x, y, z })
    }

    pub fn from_handle(handle: TextureHandle, scaling: Vec2) -> MatTex {
        MatTex::Texture { handle, scaling }
    }
}

#[derive(Clone, Copy)]
pub struct Transparency {
    pub index_of_refraction: f64,
    pub amount: f64,
}

impl Transparency {
    pub fn not_transparent() -> Self {
        Self {
            index_of_refraction: 1.0,
            amount: 0.0,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.amount > 0.0
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    pub texture: MatTex,
    pub c_diffuse: f64,
    pub c_ambient: f64,
    pub c_reflection: f64,
    pub transparency: Transparency,
}

impl Material {
    #[allow(dead_code)]
    pub fn diffuse(tex: MatTex) -> Material {
        Material {
            texture: tex,
            c_diffuse: 0.7,
            c_ambient: 0.3,
            c_reflection: 0.0,
            transparency: Transparency::not_transparent(),
        }
    }

    pub fn reflective(tex: MatTex) -> Material {
        Material {
            texture: tex,
            c_diffuse: 0.3,
            c_ambient: 0.0,
            c_reflection: 0.7,
            transparency: Transparency::not_transparent(),
        }
    }
}
