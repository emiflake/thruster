/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   texture_map.rs                                     :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/25 16:39:42 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/26 23:09:57 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TextureHandle(usize);

#[derive(Debug, Clone)]
pub struct TextureMap {
    pub textures: Vec<image::RgbImage>,
}

impl TextureMap {
    pub fn new() -> Self {
        TextureMap {
            textures: Vec::new(),
        }
    }

    pub fn load_image_from_file(
        &mut self,
        filename: &str,
    ) -> std::result::Result<TextureHandle, String> {
        let mut f = File::open(filename)
            .map_err(|_| format!("Could not open file of image {}	", filename))?;
        let reader = BufReader::new(&mut f);
        let img = image::load(reader, image::ImageFormat::PNG)
            .map_err(|_| "Could not load image as PNG")?;
        let index = self.textures.len();
        self.textures.push(img.to_rgb());
        Ok(TextureHandle(index))
    }

    pub fn get_image_by_handle(
        &self,
        handle: TextureHandle,
    ) -> std::result::Result<&image::RgbImage, String> {
        match self.textures.get(handle.0).as_ref() {
            Some(img_ref) => Ok(img_ref),
            None => Err(format!(
                "Could not get image from that handle ({})!",
                handle.0
            )),
        }
    }
}
