/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   dither.rs                                          :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/27 12:39:28 by nmartins       #+#    #+#                */
/*   Updated: 2019/08/05 16:40:29 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

use crate::algebra::Vec3;
use image::{ImageBuffer, Rgba};
#[derive(Clone)]
pub struct ColorPalette(pub Vec<Vec3>);

impl ColorPalette {
    #[allow(dead_code)]
    pub fn nearest_color(&self, ref_color: Vec3) -> Vec3 {
        fn color_diff(a: &Vec3, b: Vec3) -> u32 {
            (((b.x - a.x) * 0.3).powf(2.0)
                + ((b.y - a.y) * 0.59).powf(2.0)
                + ((b.z - a.z) * 0.11).powf(2.0)) as u32
        }

        let x = self
            .0
            .iter()
            .min_by(|a, b| color_diff(a, ref_color).cmp(&color_diff(b, ref_color)))
            .unwrap_or_else(|| &Vec3::ORIGIN);

        *x
    }

    #[allow(dead_code)]
    pub fn codam_palette() -> ColorPalette {
        ColorPalette(vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(228.0, 148.0, 57.0),
            Vec3::new(203.0, 57.0, 50.0),
            Vec3::new(217.0, 100.0, 54.0),
            Vec3::new(85.0, 160.0, 213.0),
            Vec3::new(167.0, 60.0, 131.0),
            Vec3::new(255.0, 255.0, 255.0),
        ])
    }
    #[allow(dead_code)]
    pub fn vga_palette() -> ColorPalette {
        ColorPalette(vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 170.0),
            Vec3::new(0.0, 170.0, 0.0),
            Vec3::new(0.0, 170.0, 170.0),
            Vec3::new(170.0, 0.0, 0.0),
            Vec3::new(170.0, 0.0, 170.0),
            Vec3::new(170.0, 170.0, 170.0),
            Vec3::new(85.0, 85.0, 85.0),
            Vec3::new(85.0, 85.0, 255.0),
            Vec3::new(85.0, 255.0, 85.0),
            Vec3::new(85.0, 255.0, 255.0),
            Vec3::new(255.0, 85.0, 85.0),
            Vec3::new(255.0, 85.0, 255.0),
            Vec3::new(255.0, 255.0, 0.0),
            Vec3::new(255.0, 255.0, 85.0),
            Vec3::new(255.0, 255.0, 255.0),
        ])
    }
}

pub struct Dither {
    pub palette: ColorPalette,
}

impl Dither {
    pub fn dither_image(&self, mut buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        fn add_color(pixel: &mut Rgba<u8>, delta: Vec3) {
            let curr = Vec3::from_rgba(*pixel);
            let new = (curr + delta).clamp_as_color();
            *pixel = new.to_rgba();
        }

        fn add_coords(buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, delta: Vec3) {
            if x < buf.width() && y < buf.height() {
                add_color(buf.get_pixel_mut(x, y), delta);
            }
        }

        for y in 0..buf.height() {
            for x in 0..buf.width() {
                let old_pixel = buf.get_pixel(x, y);
                let old_color = Vec3::from_rgba(*old_pixel);
                let new_color = self.palette.nearest_color(old_color);
                buf.put_pixel(x, y, new_color.to_rgba());
                let quant_error = old_color - new_color;
                add_coords(&mut buf, x + 1, y, quant_error * 7.0 / 16.0);
                add_coords(&mut buf, x - 1, y + 1, quant_error * 3.0 / 16.0);
                add_coords(&mut buf, x, y + 1, quant_error * 5.0 / 16.0);
                add_coords(&mut buf, x + 1, y + 1, quant_error * 1.0 / 16.0);
            }
        }
    }
}
