/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   material.rs                                        :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/20 19:44:22 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/21 12:17:29 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::shape::Vec3;
use image;

use std::fs::File;
use std::io::BufReader;

#[derive(Clone)]
pub enum MatTex {
	Color(Vec3),
	Image(image::RgbImage),
}

impl MatTex {
	pub fn from_color(x: f64, y: f64, z: f64) -> MatTex {
		MatTex::Color(Vec3 { x, y, z })
	}
	pub fn load_from_file(_filename: &'static str) -> std::io::Result<MatTex> {
		let mut f = File::open(_filename)?;
		let reader = BufReader::new(&mut f);
		let img = image::load(reader, image::ImageFormat::PNG)
			.map_err(|_| std::io::ErrorKind::InvalidData)?;
		Ok(MatTex::Image(img.to_rgb()))
	}
}

#[derive(Clone)]
pub struct Material {
	pub texture: MatTex,
	pub c_diffuse: f64,
	pub c_ambient: f64,
	pub c_reflection: f64,
}

impl Material {
	pub fn diffuse(tex: MatTex) -> Material {
		Material {
			texture: tex,
			c_diffuse: 0.7,
			c_ambient: 0.3,
			c_reflection: 0.0,
		}
	}

	pub fn reflective(tex: MatTex) -> Material {
		Material {
			texture: tex,
			c_diffuse: 0.3,
			c_ambient: 0.0,
			c_reflection: 0.7,
		}
	}
}
